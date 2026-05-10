<template>
  <main class="shell">
    <section class="app-head">
      <div>
        <p class="eyebrow">Artifact Lab</p>
        <h1>圣遗物 / 遗器概率实验台</h1>
      </div>
      <nav class="tabs" aria-label="功能切换">
        <button :class="{ active: view === 'items' }" type="button" @click="view = 'items'">单件模拟</button>
        <button :class="{ active: view === 'graduation' }" type="button" @click="view = 'graduation'">毕业模拟</button>
        <button :class="{ active: view === 'average' }" type="button" @click="view = 'average'">平均统计</button>
      </nav>
    </section>

    <p v-if="error" class="error-line">{{ error }}</p>

    <section v-if="view === 'items'" class="workspace three-col">
      <aside class="panel controls">
        <div class="segmented">
          <button :class="{ active: itemGame === 'genshin' }" type="button" @click="switchItemGame('genshin')">原神</button>
          <button :class="{ active: itemGame === 'hsr' }" type="button" @click="switchItemGame('hsr')">崩铁</button>
        </div>

        <label v-if="itemGame === 'genshin'">
          来源
          <select v-model="artifactForm.source" @change="handleArtifactSourceChange">
            <option value="domain">90级圣遗物副本</option>
            <option value="strongbox">合成台三合一</option>
            <option value="boss">世界/周本 Boss</option>
            <option value="elite">精英怪掉落</option>
          </select>
        </label>

        <label v-else>
          刷取入口
          <select v-model="relicForm.source" @change="handleRelicSourceChange">
            <option value="cavern">侵蚀隧洞</option>
            <option value="planar">位面饰品</option>
          </select>
        </label>
        <section class="set-picker">
          <div class="set-picker-head">
            <div>
              <span class="set-picker-label">掉落池</span>
              <strong>{{ selectedPoolCount }}/{{ activeSetPools.length }}</strong>
            </div>
            <div class="set-actions">
              <button type="button" @click="selectAllItemPools">全选</button>
              <button type="button" @click="resetItemPools">默认</button>
              <button type="button" @click="clearItemPools">清空</button>
            </div>
          </div>
          <div class="set-grid">
            <label v-for="pool in activeSetPools" :key="pool.id" class="set-option">
              <input :checked="activeItemForm.poolIds.includes(pool.id)" type="checkbox" @change="toggleItemPool(pool.id)" />
              <span>
                <strong>{{ pool.name }}</strong>
                <em>{{ describePool(pool) }}</em>
              </span>
            </label>
          </div>
        </section>

        <div class="button-row">
          <button type="button" :disabled="busy" @click="generateCurrentItem">生成</button>
          <button type="button" :disabled="busy || !currentItem || currentItem.level >= maxItemLevel" @click="enhanceCurrentItem(false)">
            强化一次
          </button>
          <button type="button" :disabled="busy || !currentItem || currentItem.level >= maxItemLevel" @click="enhanceCurrentItem(true)">
            强化到满级
          </button>
        </div>

        <section class="note-panel">
          <h2>迁移后的职责</h2>
          <p>抽取与强化全部由 FastAPI 调用 Python 模型完成，Vue 只负责参数、状态和展示。</p>
        </section>
      </aside>

      <section class="item-stage">
        <article v-if="currentItem" class="item-card">
          <header class="item-title">
            <div class="item-mark">{{ itemGame === 'genshin' ? 'A' : 'R' }}</div>
            <div>
              <h2>{{ formatSetName(currentItem.set) }}</h2>
              <div class="meta-row">
                <span>{{ slotLabel(currentItem.slot) }}</span>
                <span>+{{ currentItem.level }}</span>
                <span>{{ currentItem.rarity }}星</span>
              </div>
            </div>
          </header>

          <section class="main-stat">
            <div>
              <span>主词条</span>
              <strong>{{ statLabel(currentItem.main_stat) }}</strong>
            </div>
            <div>
              <span>当前数值</span>
              <strong>{{ formatStat(currentItem.main_stat, currentItem.main_stat_value) }}</strong>
            </div>
            <div>
              <span>初始词条</span>
              <strong>{{ currentItem.initial_substat_count }}</strong>
            </div>
          </section>

          <section class="substat-grid">
            <article v-for="substat in currentItem.substats" :key="substat.name" class="substat">
              <span>{{ statLabel(substat.name) }}</span>
              <strong>{{ formatStat(substat.name, substat.value) }}</strong>
              <em>{{ substat.roll_count }} 次：{{ substat.rolls.map((roll) => roll.toFixed(2)).join(' + ') }}</em>
            </article>
          </section>
        </article>
        <div v-else class="empty-state">点击生成，后端会返回一件 5 星装备。</div>
      </section>

      <aside class="panel stats-panel">
        <h2>强化记录</h2>
        <ol class="history">
          <li v-if="!currentItem?.enhancement_log?.length">尚未强化</li>
          <li v-for="event in currentItem?.enhancement_log ?? []" :key="`${event.level}-${event.stat}-${event.value}`">
            +{{ event.level }} {{ event.kind === 'new_substat' ? '新增' : '命中' }}
            {{ statLabel(event.stat) }}，{{ Number(event.roll).toFixed(2) }}
          </li>
        </ol>
        <div class="score-grid">
          <div class="metric compact">
            <span>双暴分</span>
            <strong>{{ (currentItem?.crit_value ?? 0).toFixed(2) }}</strong>
          </div>
          <div v-if="itemGame === 'hsr'" class="metric compact">
            <span>速度副词条</span>
            <strong>{{ (currentItem?.speed_value ?? 0).toFixed(2) }}</strong>
          </div>
        </div>
      </aside>
    </section>

    <section v-else class="workspace two-col">
      <aside class="panel controls">
        <div class="field-grid">
          <label>
            游戏
            <select v-model="graduation.game" @change="resetGraduation">
              <option value="genshin">原神圣遗物</option>
              <option value="hsr">崩铁遗器</option>
            </select>
          </label>
          <label>
            定位
            <select v-model="graduation.role" @change="resetGraduation">
              <option value="dps">主C</option>
              <option value="support">辅助</option>
            </select>
          </label>
        </div>

        <section class="control-block">
          <div class="block-head">
            <h2>有效词条</h2>
            <span>最多额外选择 3 个</span>
          </div>
          <div class="tag-grid">
            <label v-for="stat in gameData.effective" :key="stat" class="tag-option">
              <input :checked="graduation.effective_tags.includes(stat)" type="checkbox" @change="toggleTag(stat)" />
              <span>{{ statLabel(stat) }}</span>
            </label>
          </div>
        </section>

        <section class="control-block">
          <div class="block-head">
            <h2>角色面板</h2>
            <span>基础值 / 目标值 / 词条次数</span>
          </div>
          <div class="goal-grid">
            <div v-for="stat in trackedStats" :key="stat" class="goal-row">
              <div class="goal-name">{{ statLabel(stat) }}</div>
              <label>
                基础
                <input v-model.number="graduation.base[stat]" type="number" step="0.1" @input="syncGoalsAfterBaseOrMain" />
              </label>
              <label>
                面板目标
                <input v-model.number="graduation.panel_goals[stat]" type="number" step="0.1" @input="syncGoal(stat, 'panel')" />
              </label>
              <label>
                词条次数
                <input v-model.number="graduation.hit_goals[stat]" type="number" step="1" @input="syncGoal(stat, 'hits')" />
              </label>
            </div>
          </div>
        </section>

        <section class="control-block">
          <div class="block-head">
            <h2>主词条方案</h2>
            <span>只装备符合方案的部位</span>
          </div>
          <div class="field-grid">
            <label v-for="slot in slotsForGame" :key="slot">
              {{ slotLabel(slot) }}
              <select v-model="graduation.main_plan[slot]" @change="syncGoalsAfterBaseOrMain">
                <option v-for="stat in mainOptions[slot]" :key="stat" :value="stat">{{ statLabel(stat) }}</option>
              </select>
            </label>
          </div>
        </section>

        <section class="control-block">
          <div class="field-grid">
            <label>
              最大刷取次数
              <input v-model.number="graduation.max_runs" type="number" min="1" step="100" />
            </label>
            <label>
              每天刷取次数
              <input v-model.number="graduation.runs_per_day" type="number" min="1" step="1" />
            </label>
            <label v-if="view === 'average'">
              模拟次数
              <input v-model.number="graduation.sample_count" type="number" min="1" max="5000" step="50" />
            </label>
            <label v-if="view === 'average'">
              计算线程
              <input v-model.number="graduation.worker_count" type="number" min="1" :max="maxWorkerCount" step="1" />
            </label>
          </div>
        </section>

        <button type="button" :disabled="busy" @click="view === 'average' ? runAverageSimulation() : runSingleSimulation()">
          {{ busy ? '计算中...' : view === 'average' ? '开始统计' : '开始模拟' }}
        </button>
      </aside>

      <section class="main-panel">
        <section v-if="view === 'graduation'" class="summary-grid">
          <article class="metric">
            <span>停止原因</span>
            <strong>{{ singleResult?.stopReason ?? '等待' }}</strong>
          </article>
          <article class="metric">
            <span>刷取次数</span>
            <strong>{{ formatNumber(singleResult?.stopRun) }}</strong>
          </article>
          <article class="metric">
            <span>消耗天数</span>
            <strong>{{ formatNumber(singleResult ? singleResult.stopRun / singleResult.config.runsPerDay : undefined) }}</strong>
          </article>
          <article class="metric">
            <span>达成率</span>
            <strong>{{ singleResult ? `${Math.min(100, singleResult.evaluation.completion * 100).toFixed(1)}%` : '-' }}</strong>
          </article>
        </section>

        <section v-else class="summary-grid average-grid">
          <article class="metric">
            <span>达标样本</span>
            <strong>{{ averageResult ? `${averageResult.success}/${averageResult.total}` : averageProgressText }}</strong>
          </article>
          <article class="metric">
            <span>达标率</span>
            <strong>{{ averageResult ? `${(averageResult.successRate * 100).toFixed(1)}%` : '-' }}</strong>
          </article>
          <article class="metric">
            <span>平均刷取</span>
            <strong>{{ formatRuns(averageResult?.average) }}</strong>
          </article>
          <article class="metric">
            <span>P90 刷取</span>
            <strong>{{ formatRuns(averageResult?.p90) }}</strong>
          </article>
        </section>

        <section v-if="view === 'graduation'" class="chart-panel">
          <div class="block-head">
            <h2>面板达成曲线</h2>
            <div class="chart-tabs" role="group" aria-label="横轴">
              <button type="button" :class="{ active: chartAxis === 'runs' }" @click="setChartAxis('runs')">刷取次数</button>
              <button type="button" :class="{ active: chartAxis === 'days' }" @click="setChartAxis('days')">天数</button>
            </div>
          </div>
          <div class="chart-tools" aria-label="曲线缩放控制">
            <div class="chart-zoom-buttons">
              <button type="button" :disabled="chartZoom <= 1" @click="zoomChart(0.5)">缩小</button>
              <button type="button" :disabled="chartZoom >= maxChartZoom" @click="zoomChart(2)">放大</button>
              <button type="button" @click="resetChartView">重置</button>
            </div>
            <label class="chart-pan">
              <span class="chart-view-label">{{ chartViewLabel }}</span>
              <input
                v-model.number="pendingChartPan"
                :disabled="chartZoom <= 1"
                type="range"
                min="0"
                max="1"
                step="0.001"
                @change="commitChartPan"
                @pointerup="commitChartPan"
                @keyup.left="commitChartPan"
                @keyup.right="commitChartPan"
                @keyup.home="commitChartPan"
                @keyup.end="commitChartPan"
              />
            </label>
          </div>
          <p class="chart-status">
            {{ singleResult?.history?.length ?? 0 }} 个采样点 · 曲线仅标出达成节点，最终件在下方时间轴展示
          </p>
          <svg class="chart" viewBox="0 0 720 300" role="img" aria-label="毕业模拟曲线" @wheel.prevent="handleChartWheel">
            <defs>
              <clipPath id="completion-chart-clip">
                <rect :x="chartFrame.left" :y="chartFrame.top - 10" :width="chartWidth" :height="chartHeight + 20" />
              </clipPath>
            </defs>
            <line :x1="chartFrame.left" :y1="chartFrame.top" :x2="chartFrame.left" :y2="chartFrame.bottom" />
            <line :x1="chartFrame.left" :y1="chartFrame.bottom" :x2="chartFrame.right" :y2="chartFrame.bottom" />
            <g v-for="line in chartGridLines" :key="line.label">
              <line class="guide-line" :x1="chartFrame.left" :y1="line.y" :x2="chartFrame.right" :y2="line.y" />
              <text x="12" :y="line.y + 4">{{ line.label }}</text>
            </g>
            <g v-for="tick in chartXAxisTicks" :key="tick.label">
              <line class="tick-line" :x1="tick.x" :y1="chartFrame.bottom" :x2="tick.x" :y2="chartFrame.bottom + 6" />
              <text :x="tick.x" :y="chartFrame.bottom + 23" text-anchor="middle">{{ tick.label }}</text>
            </g>
            <g clip-path="url(#completion-chart-clip)">
              <path class="completion-path" :d="completionChartPath" />
              <g v-for="threshold in chartThresholdMarkers" :key="threshold.key">
                <line class="target-line" :x1="threshold.x" :y1="threshold.y" :x2="threshold.x" :y2="chartFrame.bottom" />
                <g class="chart-hit" role="button" tabindex="0" @mouseenter="hoverChartNode(threshold.key)" @mouseleave="clearChartHover" @focus="hoverChartNode(threshold.key)" @blur="clearChartHover" @click="selectChartNode(threshold.key)" @keydown.enter.prevent="selectChartNode(threshold.key)" @keydown.space.prevent="selectChartNode(threshold.key)">
                  <circle class="threshold-node" :class="{ active: activeChartKey === threshold.key || hoveredChartKey === threshold.key }" :cx="threshold.x" :cy="threshold.y" r="7" />
                </g>
              </g>
            </g>
            <g v-if="activeChartDetail" class="chart-tooltip">
              <rect :x="activeChartDetail.boxX" :y="activeChartDetail.boxY" width="178" height="58" rx="7" />
              <text :x="activeChartDetail.boxX + 10" :y="activeChartDetail.boxY + 20">{{ activeChartDetail.title }}</text>
              <text :x="activeChartDetail.boxX + 10" :y="activeChartDetail.boxY + 40">{{ activeChartDetail.detail }}</text>
            </g>
            <text :x="chartFrame.left" y="290">{{ chartAxis === 'runs' ? '刷取次数' : '天数' }}</text>
          </svg>
          <div v-if="chartFinalPieceMarkers.length" class="final-piece-rail" aria-label="最终件出现时间轴">
            <div class="final-piece-rail-head">
              <strong>最终件出现</strong>
              <span>卡点快照不叠加到曲线上</span>
            </div>
            <div class="final-piece-track">
              <button
                v-for="marker in chartFinalPieceMarkers"
                :key="marker.key"
                class="final-piece-pin"
                :class="{ active: activeChartKey === marker.key || hoveredChartKey === marker.key }"
                :style="{ '--x': `${marker.leftPct}%`, '--lane': marker.lane }"
                :title="`${marker.title} · ${marker.detail}`"
                type="button"
                @mouseenter="hoverChartNode(marker.key)"
                @mouseleave="clearChartHover"
                @focus="hoverChartNode(marker.key)"
                @blur="clearChartHover"
                @click="selectChartNode(marker.key)"
              >
                <span>{{ slotLabel(marker.slot) }}</span>
              </button>
            </div>
          </div>
        </section>

        <section v-if="view === 'graduation'" class="table-panel">
          <div class="block-head">
            <h2>达成节点</h2>
            <span>75% / 90% / 95% / 98% / 100%</span>
          </div>
          <div class="threshold-grid">
            <article
              v-for="threshold in singleThresholdStats"
              :key="threshold.key"
              :ref="(el) => setThresholdRef(threshold.chartKey, el)"
              class="threshold-card"
              :class="{ active: activeChartKey === threshold.chartKey }"
              role="button"
              tabindex="0"
              @mouseenter="hoverChartNode(threshold.chartKey)"
              @mouseleave="clearChartHover"
              @focus="hoverChartNode(threshold.chartKey)"
              @blur="clearChartHover"
              @click="selectChartNode(threshold.chartKey)"
              @keydown.enter.prevent="selectChartNode(threshold.chartKey)"
              @keydown.space.prevent="selectChartNode(threshold.chartKey)"
            >
              <span>{{ threshold.label }}</span>
              <strong>{{ threshold.reached ? formatNumber(threshold.run) : '-' }}</strong>
              <em>{{ threshold.reached ? `${formatNumber(threshold.days)} 天` : '未达到' }}</em>
              <div class="threshold-mini-track">
                <i :style="{ width: `${threshold.ratio * 100}%` }"></i>
              </div>
            </article>
          </div>
        </section>

        <section v-if="view === 'graduation'" class="table-panel">
          <div class="block-head">
            <h2>当前装备</h2>
            <span>{{ equippedCount }}/{{ slotsForGame.length }} 部位</span>
          </div>
          <div class="piece-grid">
            <article v-for="slot in slotsForGame" :key="slot" class="piece-card">
              <span>{{ slotLabel(slot) }}</span>
              <template v-if="singleResult?.build?.[slot]">
                <strong>{{ statLabel(singleResult.build[slot].mainStat) }}</strong>
                <em>双暴分 {{ coreCritValue(singleResult.build[slot]).toFixed(1) }}</em>
                <small>{{ describeItem(singleResult.build[slot]) }}</small>
              </template>
              <strong v-else>-</strong>
            </article>
          </div>
        </section>

        <section v-if="view === 'graduation'" class="table-panel">
          <div class="block-head">
            <h2>当前面板</h2>
            <span>面板 / 词条总次数</span>
          </div>
          <div class="stat-table">
            <article v-for="stat in singleResult?.config?.stats ?? trackedStats" :key="stat" class="stat-card">
              <div class="stat-name">{{ statLabel(stat) }}</div>
              <div class="stat-value">{{ formatStat(stat, singleResult?.evaluation?.panel?.[stat] ?? graduation.base[stat] ?? 0) }}</div>
              <div class="stat-meta">
                面板目标 {{ formatStat(stat, singleResult?.config?.panelGoals?.[stat] ?? graduation.panel_goals[stat] ?? 0) }} ·
                词条次数 {{ singleResult?.evaluation?.hits?.[stat] ?? 0 }}/{{ singleResult?.config?.hitGoals?.[stat] ?? graduation.hit_goals[stat] ?? 0 }}
              </div>
            </article>
          </div>
        </section>

        <section v-if="view === 'graduation'" class="table-panel">
          <div class="block-head">
            <h2>里程碑</h2>
            <span>{{ visibleMilestoneList.length }} 个节点</span>
          </div>
          <div v-if="!visibleMilestoneList.length" class="empty-line">暂无里程碑</div>
          <article
            v-for="milestone in visibleMilestoneList"
            :key="milestone.chartKey"
            :ref="(el) => setMilestoneRef(milestone.chartKey, el)"
            class="milestone"
            :class="{ active: activeChartKey === milestone.chartKey }"
            role="button"
            tabindex="0"
            @mouseenter="hoverChartNode(milestone.chartKey)"
            @mouseleave="clearChartHover"
            @focus="hoverChartNode(milestone.chartKey)"
            @blur="clearChartHover"
            @click="selectChartNode(milestone.chartKey)"
            @keydown.enter.prevent="selectChartNode(milestone.chartKey)"
            @keydown.space.prevent="selectChartNode(milestone.chartKey)"
          >
            <div class="milestone-time">
              <strong>{{ formatNumber(milestone.run) }}</strong>
              <span>刷取次数</span>
              <em>{{ formatNumber(milestone.days) }} 天</em>
            </div>
            <div class="milestone-body">
              <div class="milestone-head">
                <h3>{{ milestone.title }}</h3>
                <span>{{ milestoneTypeName(milestone.type) }}</span>
              </div>
              <div class="milestone-stats">
                <div v-for="stat in singleResult.config.stats" :key="stat" class="milestone-stat">
                  <span>{{ statLabel(stat) }}</span>
                  <strong>{{ formatStat(stat, milestone.panel[stat] || 0) }}</strong>
                  <em>{{ milestone.hits[stat] || 0 }} 次</em>
                </div>
              </div>
              <p v-if="milestone.type === 'plateau'" class="milestone-note">
                达成率停滞 {{ formatNumber(milestone.stallRuns) }} 次，约 {{ formatNumber(milestone.stallDays) }} 天。
              </p>
              <p v-if="milestone.item" class="milestone-note">
                {{ slotLabel(milestone.item.slot) }} · {{ statLabel(milestone.item.mainStat) }} · 双暴分 {{ milestone.item.critValue.toFixed(1) }}
              </p>
              <details v-if="milestone.buildSnapshot?.length" class="milestone-build">
                <summary>当时装备组成</summary>
                <div class="milestone-build-grid">
                  <div v-for="piece in milestone.buildSnapshot" :key="piece.slot" class="milestone-piece" :class="{ empty: piece.empty }">
                    <span>{{ slotLabel(piece.slot) }}</span>
                    <strong>{{ piece.empty ? '空' : `${statLabel(piece.mainStat)} · 双暴分 ${piece.critValue.toFixed(1)}` }}</strong>
                    <em v-if="!piece.empty">{{ piece.substats }}</em>
                  </div>
                </div>
              </details>
            </div>
          </article>
        </section>

        <section v-if="view === 'average' && !averageResult" class="chart-panel average-empty-panel">
          <div class="empty-hero">
            <strong>{{ busy ? '统计正在路上' : '先设定一个毕业目标' }}</strong>
            <p>
              {{ busy
                ? `已完成 ${averageProgress.done}/${averageProgress.total || graduation.sample_count} 个样本，结果会边跑边刷新。`
                : '左侧调整目标、样本数和线程后点击开始统计。平均值看总体成本，P90/P95 更适合估算保守资源。' }}
            </p>
            <div v-if="busy" class="progress-track" aria-label="平均统计进度">
              <i :style="{ width: `${averageProgressPercent}%` }"></i>
            </div>
            <div class="empty-tips">
              <span>平均刷取 = 达标样本的平均次数</span>
              <span>P90 = 90% 样本能在该次数内完成</span>
              <span>未达标样本会计入风险，不拉低平均值</span>
            </div>
          </div>
        </section>

        <section v-if="view === 'average' && averageResult" class="chart-panel average-visual-panel">
          <div class="block-head">
            <h2>达成率分析</h2>
            <span>{{ averageResult?.done ?? 0 }}/{{ averageResult?.total ?? graduation.sample_count }} 样本</span>
          </div>
          <div class="average-visual-grid">
            <article class="visual-card">
              <div class="visual-head">
                <strong>分档达成漏斗</strong>
                <span>越靠后越接近完整毕业</span>
              </div>
              <div class="funnel-chart">
                <div v-for="bar in averageRateBars" :key="bar.key" class="funnel-row">
                  <span>{{ bar.label }}</span>
                  <div class="funnel-track">
                    <i :style="{ width: `${bar.ratePercent}%` }"></i>
                  </div>
                  <strong>{{ bar.rateText }}</strong>
                </div>
              </div>
            </article>

            <article class="visual-card">
              <div class="visual-head">
                <strong>刷取成本阶梯</strong>
                <span>达到每档平均要刷几次</span>
              </div>
              <svg class="mini-chart" viewBox="0 0 720 260" role="img" aria-label="平均刷取成本阶梯">
                <line x1="48" y1="30" x2="48" y2="214" />
                <line x1="48" y1="214" x2="690" y2="214" />
                <g v-for="bar in averageCostBars" :key="bar.key">
                  <rect :x="bar.x" :y="bar.y" width="72" :height="bar.height" rx="6" />
                  <text :x="bar.x + 36" :y="bar.y - 8" text-anchor="middle">{{ bar.valueText }}</text>
                  <text :x="bar.x + 36" y="238" text-anchor="middle">{{ bar.label }}</text>
                </g>
              </svg>
            </article>
          </div>

          <div class="average-visual-grid lower">
            <article class="visual-card wide">
              <div class="visual-head">
                <strong>100% 达成分布</strong>
                <span>{{ compactAverageBuckets.length }} 个有效区间</span>
              </div>
              <svg class="histogram-chart" viewBox="0 0 720 260" role="img" aria-label="100%达成分布">
                <line x1="48" y1="30" x2="48" y2="214" />
                <line x1="48" y1="214" x2="690" y2="214" />
                <g v-for="bucket in averageBucketBars" :key="bucket.key">
                  <rect :x="bucket.x" :y="bucket.y" :width="bucket.width" :height="bucket.height" rx="5" />
                  <text v-if="bucket.count" :x="bucket.x + bucket.width / 2" :y="bucket.y - 6" text-anchor="middle">{{ bucket.count }}</text>
                </g>
              </svg>
            </article>

            <article class="visual-card insight-card">
              <div class="visual-head">
                <strong>风险提示</strong>
                <span>来自当前样本</span>
              </div>
              <div class="insight-list">
                <div v-for="item in averageInsightCards" :key="item.label">
                  <span>{{ item.label }}</span>
                  <strong>{{ item.value }}</strong>
                  <em>{{ item.detail }}</em>
                </div>
              </div>
            </article>
          </div>
        </section>

        <section v-if="view === 'average'" class="table-panel">
          <div class="block-head">
            <h2>分位统计</h2>
            <span v-if="averageResult">{{ averageResult.failed }} 个样本未在上限内达标</span>
            <span v-else>等待统计结果</span>
          </div>
          <div class="analysis-grid">
            <article class="analysis-card">
              <span>中位数</span>
              <strong>{{ formatRuns(averageResult?.median) }}</strong>
            </article>
            <article class="analysis-card">
              <span>P95 刷取</span>
              <strong>{{ formatRuns(averageResult?.p95) }}</strong>
            </article>
            <article class="analysis-card">
              <span>平均天数</span>
              <strong>{{ formatDays(averageResult?.averageDays) }}</strong>
            </article>
          </div>
          <div class="threshold-table">
            <article v-for="threshold in averageThresholdStats" :key="threshold.key" class="threshold-row">
              <div>
                <span>{{ threshold.label }} 达成率</span>
                <strong>{{ `${(threshold.rate * 100).toFixed(1)}%` }}</strong>
              </div>
              <div>
                <span>样本数</span>
                <strong>{{ `${threshold.reached}/${averageResult?.done ?? 0}` }}</strong>
              </div>
              <div>
                <span>平均刷取</span>
                <strong>{{ formatRuns(threshold.average) }}</strong>
              </div>
              <div>
                <span>P90 刷取</span>
                <strong>{{ formatRuns(threshold.p90) }}</strong>
              </div>
            </article>
          </div>
          <div class="bucket-table">
            <div v-for="bucket in compactAverageBuckets" :key="bucket.key" class="bucket-row">
              <span>{{ bucket.label }}</span>
              <strong>{{ formatNumber(bucket.count) }}</strong>
              <em>{{ averageResult?.done ? ((bucket.count / averageResult.done) * 100).toFixed(1) : '0.0' }}%</em>
            </div>
          </div>
        </section>
      </section>
    </section>
  </main>
</template>

<script setup>
import { computed, onMounted, onUnmounted, reactive, ref } from "vue";
import {
  enhanceArtifact,
  enhanceRelic,
  generateArtifact,
  generateRelic,
  getMeta,
} from "./api";
import {
  COMPLETION_THRESHOLDS,
  convertGoalValue,
  critValue as coreCritValue,
  DEFAULT_MAIN_PLAN,
  describeSubstats as describeItem,
  formatNumber,
  formatStatValue,
  GAME_STATS,
  MAIN_OPTIONS,
  milestoneTypeLabel as milestoneTypeName,
  simulate,
  slotList,
  SLOT_LABELS,
  STAT_LABELS,
  summarizeAverage,
  trackedStats as coreTrackedStats,
  visibleMilestones,
} from "./sim/graduationCore";
import { flattenPools, SET_LABELS, SET_POOLS } from "./setPools";

const view = ref("items");
const busy = ref(false);
const error = ref("");
const meta = ref(null);
const itemGame = ref("genshin");
const currentItem = ref(null);
const singleResult = ref(null);
const averageResult = ref(null);
const averageProgress = reactive({ done: 0, total: 0 });
const lastGoalEditKind = ref("hits");
const chartAxis = ref("runs");
const chartZoom = ref(1);
const chartPan = ref(0);
const pendingChartPan = ref(0);
const activeChartKey = ref("");
const hoveredChartKey = ref("");
const milestoneRefs = new Map();
let averageWorkers = [];

const artifactForm = reactive({
  source: "domain",
  poolIds: [],
});
const relicForm = reactive({
  source: "cavern",
  poolIds: [],
});
const graduation = reactive({
  game: "genshin",
  role: "dps",
  effective_tags: [],
  base: {},
  panel_goals: {},
  hit_goals: {},
  main_plan: {},
  max_runs: 10000,
  runs_per_day: 8,
  sample_count: 200,
  worker_count: Math.min(4, Math.max(1, navigator.hardwareConcurrency || 1)),
});

const activeItemForm = computed(() => (itemGame.value === "genshin" ? artifactForm : relicForm));
const maxItemLevel = computed(() => (itemGame.value === "genshin" ? 20 : 15));
const labels = computed(() => ({
  slots: { ...(meta.value?.labels?.slots ?? {}), ...SLOT_LABELS },
  stats: { ...(meta.value?.labels?.stats ?? {}), ...STAT_LABELS },
}));
const gameData = computed(() => GAME_STATS[graduation.game] ?? { effective: [], base: {}, hitTargets: {}, panelTargets: {} });
const slotsForGame = computed(() =>
  graduation.game === "genshin" ? ["flower", "plume", "sands", "goblet", "circlet"] : ["head", "hands", "body", "feet", "planar_sphere", "link_rope"],
);
const mainOptions = computed(() => MAIN_OPTIONS[graduation.game] ?? {});
const trackedStats = computed(() =>
  graduation.role === "dps" ? ["crit_rate", "crit_dmg", ...graduation.effective_tags] : [...graduation.effective_tags],
);
const activeSetPools = computed(() => {
  if (itemGame.value === "genshin") {
    return SET_POOLS.genshin[artifactForm.source === "domain" ? "domain" : "other"];
  }
  return SET_POOLS.hsr[relicForm.source] ?? [];
});
const selectedPoolCount = computed(() => activeItemForm.value.poolIds.length);
const selectedItemSets = computed(() => {
  const poolIds = new Set(activeItemForm.value.poolIds);
  const selectedPools = activeSetPools.value.filter((pool) => poolIds.has(pool.id));
  return flattenPools(selectedPools);
});
const equippedCount = computed(() => Object.values(singleResult.value?.build ?? {}).filter(Boolean).length);
const visibleMilestoneList = computed(() =>
  singleResult.value
    ? visibleMilestones(singleResult.value).map((milestone, index) => ({
        ...milestone,
        chartKey: milestoneKey(milestone, index),
      }))
    : [],
);
const maxWorkerCount = computed(() => Math.max(1, Math.min(16, navigator.hardwareConcurrency || 4)));
const completionThresholds = COMPLETION_THRESHOLDS;
const chartFrame = Object.freeze({ left: 52, right: 690, top: 28, bottom: 236 });
const chartWidth = chartFrame.right - chartFrame.left;
const chartHeight = chartFrame.bottom - chartFrame.top;
const chartYBreakpoints = Object.freeze([
  { value: 0, position: 1 },
  { value: 0.5, position: 0.72 },
  { value: 0.75, position: 0.48 },
  { value: 0.9, position: 0.25 },
  { value: 0.95, position: 0.14 },
  { value: 0.98, position: 0.07 },
  { value: 1, position: 0 },
]);
const chartScale = computed(() => {
  const result = singleResult.value;
  const history = result?.history ?? [];
  const axis = chartAxis.value;
  const rawMax = Math.max(
    ...history.map((point) => Number(point[axis]) || 0),
    ...visibleMilestoneList.value.map((point) => Number(point[axis]) || 0),
    ...Object.values(result?.thresholdRuns ?? {})
      .filter(Number.isFinite)
      .map((run) => (axis === "days" ? run / result.config.runsPerDay : run)),
    1,
  );
  const maxX = niceCeil(rawMax);
  return { axis, maxX, easingPower: 0.62 };
});
const maxChartZoom = 16;
const chartWindow = computed(() => {
  const maxX = chartScale.value.maxX || 1;
  const zoom = Math.max(1, Math.min(maxChartZoom, Number(chartZoom.value) || 1));
  const width = maxX / zoom;
  const start = (maxX - width) * Math.max(0, Math.min(1, Number(chartPan.value) || 0));
  return {
    start,
    end: start + width,
    width,
    zoom,
  };
});
const chartViewLabel = computed(() => {
  const unit = chartScale.value.axis === "runs" ? "次" : "天";
  const start = formatNumber(chartWindow.value.start);
  const end = formatNumber(chartWindow.value.end);
  return chartZoom.value > 1 ? `${start}-${end} ${unit} · ${chartZoom.value.toFixed(1)}x` : `全局视图 · 1.0x`;
});
function chartRawX(value) {
  const safeValue = Math.max(0, Number(value) || 0);
  const ratio = Math.min(1, safeValue / chartScale.value.maxX);
  return ratio ** chartScale.value.easingPower;
}
function chartX(value) {
  const raw = chartRawX(value);
  const windowStart = chartRawX(chartWindow.value.start);
  const windowEnd = chartRawX(chartWindow.value.end);
  const ratio = (raw - windowStart) / Math.max(0.0001, windowEnd - windowStart);
  return chartFrame.left + ratio * chartWidth;
}
function chartValueInWindow(value) {
  const numericValue = Number(value) || 0;
  return numericValue >= chartWindow.value.start && numericValue <= chartWindow.value.end;
}
function chartY(completion) {
  const value = Math.max(0, Math.min(1, Number(completion) || 0));
  for (let index = 1; index < chartYBreakpoints.length; index += 1) {
    const prev = chartYBreakpoints[index - 1];
    const next = chartYBreakpoints[index];
    if (value <= next.value) {
      const localRatio = (value - prev.value) / Math.max(0.0001, next.value - prev.value);
      const position = prev.position + (next.position - prev.position) * localRatio;
      return chartFrame.top + position * chartHeight;
    }
  }
  return chartFrame.top;
}
const chartGridLines = computed(() =>
  chartYBreakpoints.map((tick) => ({
    label: `${Math.round(tick.value * 100)}%`,
    y: chartY(tick.value),
  })),
);
const chartXAxisTicks = computed(() =>
  [0, 0.25, 0.5, 0.75, 1].map((ratio) => ({
    label: formatNumber(chartWindow.value.start + chartWindow.value.width * ratio),
    x: chartFrame.left + chartWidth * ratio,
  })),
);
const chartPoints = computed(() => {
  const result = singleResult.value;
  if (!result?.history?.length) return [];
  const axis = chartScale.value.axis;
  const points = result.history
    .filter((point) => chartValueInWindow(point[axis]))
    .map((point) => ({
      ...point,
      x: chartX(point[axis]),
      y: chartY(point.completion),
    }));
  const before = [...result.history].reverse().find((point) => Number(point[axis]) < chartWindow.value.start);
  const after = result.history.find((point) => Number(point[axis]) > chartWindow.value.end);
  if (before) {
    points.unshift({
      ...before,
      x: chartFrame.left,
      y: chartY(before.completion),
    });
  }
  if (after) {
    points.push({
      ...after,
      x: chartFrame.right,
      y: chartY(after.completion),
    });
  }
  return points;
});
const completionChartPath = computed(() => {
  if (!chartPoints.value.length) return "";
  return chartPoints.value
    .map((point, pointIndex) => {
      return `${pointIndex === 0 ? "M" : "L"} ${point.x.toFixed(1)} ${point.y.toFixed(1)}`;
    })
    .join(" ");
});
const singleThresholdStats = computed(() => {
  const result = singleResult.value;
  return completionThresholds.map((threshold) => {
    const run = result?.thresholdRuns?.[threshold.key];
    return {
      ...threshold,
      chartKey: `threshold-${threshold.key}`,
      reached: Number.isFinite(run),
      run,
      days: Number.isFinite(run) && result ? run / result.config.runsPerDay : NaN,
    };
  });
});
const chartThresholdMarkers = computed(() =>
  singleThresholdStats.value
    .filter((threshold) => threshold.reached)
    .map((threshold) => {
      const axisValue = chartScale.value.axis === "days" ? threshold.days : threshold.run;
      return {
        ...threshold,
        key: threshold.chartKey,
        x: chartX(axisValue),
        y: chartY(threshold.ratio),
        title: `${threshold.label} 达成`,
        detail: `${formatNumber(threshold.run)} 次 / ${formatNumber(threshold.days)} 天`,
      };
    })
    .filter((threshold) => chartValueInWindow(chartScale.value.axis === "days" ? threshold.days : threshold.run)),
);
const finalPieceMilestoneList = computed(() =>
  visibleMilestoneList.value.filter((milestone) => milestone.type === "equip_first" || milestone.type === "equip_upgrade"),
);
const chartFinalPieceMarkers = computed(() =>
  finalPieceMilestoneList.value
    .map((milestone) => {
      const axisValue = chartScale.value.axis === "days" ? milestone.days : milestone.run;
      const leftPct = ((Number(axisValue) || 0) - chartWindow.value.start) / Math.max(0.0001, chartWindow.value.width) * 100;
      return {
        ...milestone,
        key: milestone.chartKey,
        axisValue,
        x: chartX(axisValue),
        leftPct: Math.min(100, Math.max(0, leftPct)),
        detail: `${formatNumber(milestone.run)} 次 / ${formatNumber(milestone.days)} 天`,
      };
    })
    .filter((milestone) => chartValueInWindow(milestone.axisValue))
    .map((milestone, index) => ({
      ...milestone,
      lane: index % 2,
    })),
);
const chartInteractiveMarkers = computed(() => [...chartThresholdMarkers.value, ...chartFinalPieceMarkers.value]);
const activeChartDetail = computed(() => {
  const marker = chartInteractiveMarkers.value.find((item) => item.key === hoveredChartKey.value);
  if (!marker) return null;
  const boxX = Math.min(chartFrame.right - 178, Math.max(chartFrame.left + 8, marker.x + 12));
  const boxY = Math.max(chartFrame.top + 8, marker.y - 68);
  return {
    ...marker,
    boxX,
    boxY,
    title: marker.title,
    detail: marker.detail,
  };
});
const averageThresholdStats = computed(() => averageResult.value?.thresholds ?? []);
const averageProgressPercent = computed(() => {
  const total = averageProgress.total || graduation.sample_count || 1;
  return Math.max(0, Math.min(100, (averageProgress.done / total) * 100));
});
const averageProgressText = computed(() =>
  busy.value && averageProgress.total ? `${averageProgress.done}/${averageProgress.total}` : "-",
);
const averageRateBars = computed(() =>
  averageThresholdStats.value.map((threshold) => {
    const ratePercent = Math.max(0, Math.min(100, threshold.rate * 100));
    return {
      ...threshold,
      ratePercent,
      rateText: `${ratePercent.toFixed(1)}%`,
    };
  }),
);
const averageCostBars = computed(() => {
  const bars = averageThresholdStats.value.filter((threshold) => Number.isFinite(threshold.average));
  const maxAverage = Math.max(...bars.map((threshold) => threshold.average), 1);
  return bars.map((threshold, index) => {
    const height = Math.max(2, (threshold.average / maxAverage) * 150);
    return {
      ...threshold,
      x: 72 + index * 122,
      y: 214 - height,
      height,
      valueText: formatNumber(threshold.average),
    };
  });
});
const averageBucketBars = computed(() => {
  const buckets = compactAverageBuckets.value;
  const maxCount = Math.max(...buckets.map((bucket) => bucket.count), 1);
  return buckets.map((bucket, index) => {
    const totalWidth = 608;
    const gap = buckets.length > 1 ? 12 : 0;
    const width = Math.max(34, (totalWidth - gap * (buckets.length - 1)) / Math.max(1, buckets.length));
    const height = bucket.count ? Math.max(3, (bucket.count / maxCount) * 150) : 0;
    return {
      ...bucket,
      x: 62 + index * (width + gap),
      y: 214 - height,
      width,
      height,
    };
  });
});
const compactAverageBuckets = computed(() => {
  const buckets = averageResult.value?.buckets ?? [];
  const nonZero = buckets.filter((bucket) => bucket.count > 0);
  const source = nonZero.length ? nonZero : buckets.slice(0, 1);
  const visible = source.slice(0, 6).map((bucket) => ({
    ...bucket,
    key: `${bucket.start}-${bucket.end}`,
    label: `${formatNumber(bucket.start)}-${formatNumber(bucket.end)} 次`,
  }));
  const rest = source.slice(6);
  if (rest.length) {
    const start = rest[0].start;
    const end = rest[rest.length - 1].end;
    visible.push({
      start,
      end,
      count: rest.reduce((total, bucket) => total + bucket.count, 0),
      key: `${start}-${end}`,
      label: `${formatNumber(start)}-${formatNumber(end)} 次`,
    });
  }
  return visible;
});
const averageInsightCards = computed(() => {
  const thresholds = averageThresholdStats.value;
  const t90 = thresholds.find((threshold) => threshold.key === "90");
  const t100 = thresholds.find((threshold) => threshold.key === "100");
  const tailCost = Number.isFinite(t90?.average) && Number.isFinite(t100?.average) ? t100.average - t90.average : NaN;
  const tailRatio = Number.isFinite(averageResult.value?.p95) && Number.isFinite(averageResult.value?.median) && averageResult.value.median > 0
    ? averageResult.value.p95 / averageResult.value.median
    : NaN;
  return [
    {
      label: "100% 未达标风险",
      value: t100 ? `${((1 - t100.rate) * 100).toFixed(1)}%` : "-",
      detail: "在上限内没有完整毕业的样本占比",
    },
    {
      label: "最后 10% 提升成本",
      value: formatNumber(tailCost),
      detail: "最后一段通常最吃资源",
    },
    {
      label: "尾部波动",
      value: Number.isFinite(tailRatio) ? `${tailRatio.toFixed(2)}x` : "-",
      detail: "P95 相对中位数的放大倍数",
    },
  ];
});

function niceCeil(value) {
  if (!Number.isFinite(value) || value <= 0) return 1;
  const power = 10 ** Math.floor(Math.log10(value));
  const scaled = value / power;
  const step = scaled <= 1 ? 1 : scaled <= 2 ? 2 : scaled <= 5 ? 5 : 10;
  return step * power;
}

function milestoneKey(milestone, index) {
  const slot = milestone.slot ?? "all";
  return `milestone-${milestone.type}-${milestone.run}-${slot}-${index}`;
}

function setThresholdRef(key, el) {
  setChartRef(key, el);
}

function setMilestoneRef(key, el) {
  setChartRef(key, el);
}

function setChartRef(key, el) {
  if (!key) return;
  if (el) milestoneRefs.set(key, el);
  else milestoneRefs.delete(key);
}

function hoverChartNode(key) {
  hoveredChartKey.value = key;
}

function clearChartHover() {
  hoveredChartKey.value = "";
}

function resetChartView() {
  chartZoom.value = 1;
  chartPan.value = 0;
  pendingChartPan.value = 0;
  clearChartHover();
}

function setChartAxis(axis) {
  chartAxis.value = axis;
  resetChartView();
}

function zoomChart(multiplier) {
  const nextZoom = Math.max(1, Math.min(maxChartZoom, chartZoom.value * multiplier));
  if (Math.abs(nextZoom - chartZoom.value) < 0.001) return;
  chartZoom.value = nextZoom;
  if (nextZoom <= 1) {
    chartPan.value = 0;
    pendingChartPan.value = 0;
  }
  clearChartHover();
}

function handleChartWheel(event) {
  zoomChart(event.deltaY < 0 ? 1.35 : 1 / 1.35);
}

function commitChartPan() {
  chartPan.value = Math.max(0, Math.min(1, Number(pendingChartPan.value) || 0));
  clearChartHover();
}

function selectChartNode(key) {
  activeChartKey.value = key;
  clearChartHover();
  const el = milestoneRefs.get(key);
  if (el) {
    el.scrollIntoView({ behavior: "smooth", block: "center" });
  }
}

function statLabel(stat) {
  return labels.value.stats[stat] ?? stat;
}

function slotLabel(slot) {
  return labels.value.slots[slot] ?? slot;
}

function formatSetName(setName) {
  if (SET_LABELS[setName]) return SET_LABELS[setName];
  return setName
    .split("_")
    .filter(Boolean)
    .map((part) => part[0]?.toUpperCase() + part.slice(1))
    .join(" ");
}

function describePool(pool) {
  return pool.sets.map(formatSetName).join(" / ");
}

function formatStat(stat, value) {
  return formatStatValue(stat, Number(value));
}

function formatRuns(value) {
  return Number.isFinite(value) ? `${formatNumber(value)} 次` : "-";
}

function formatDays(value) {
  return Number.isFinite(value) ? `${formatNumber(value)} 天` : "-";
}

function runtimeSeed(prefix) {
  return `${prefix}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

function switchItemGame(game) {
  itemGame.value = game;
  currentItem.value = null;
  resetItemPools();
}

function toggleItemPool(poolId) {
  const selected = activeItemForm.value.poolIds;
  if (selected.includes(poolId)) {
    activeItemForm.value.poolIds = selected.filter((item) => item !== poolId);
    return;
  }
  activeItemForm.value.poolIds = [...selected, poolId];
}

function selectAllItemPools() {
  activeItemForm.value.poolIds = activeSetPools.value.map((pool) => pool.id);
}

function clearItemPools() {
  activeItemForm.value.poolIds = [];
}

function resetItemPools() {
  activeItemForm.value.poolIds = activeSetPools.value.map((pool) => pool.id);
}

function handleRelicSourceChange() {
  resetItemPools();
}

function handleArtifactSourceChange() {
  resetItemPools();
}

async function withBusy(task) {
  busy.value = true;
  error.value = "";
  try {
    await task();
  } catch (err) {
    error.value = err.message ?? String(err);
  } finally {
    busy.value = false;
  }
}

async function generateCurrentItem() {
  await withBusy(async () => {
    const form = activeItemForm.value;
    if (!selectedItemSets.value.length) {
      throw new Error("请至少选择一个掉落池。");
    }
    const payload = { ...form, sets: [...selectedItemSets.value], level: 0 };
    currentItem.value = itemGame.value === "genshin" ? await generateArtifact(payload) : await generateRelic(payload);
  });
}

async function enhanceCurrentItem(toMax) {
  if (!currentItem.value) return;
  await withBusy(async () => {
    const payload = {
      item: currentItem.value,
      to_max: toMax,
    };
    currentItem.value = itemGame.value === "genshin" ? await enhanceArtifact(payload) : await enhanceRelic(payload);
  });
}

function toggleTag(stat) {
  if (graduation.effective_tags.includes(stat)) {
    graduation.effective_tags = graduation.effective_tags.filter((item) => item !== stat);
  } else if (graduation.effective_tags.length < 3) {
    graduation.effective_tags = [...graduation.effective_tags, stat];
  }
  ensureGraduationFields();
}

function resetGraduation() {
  const defaults = graduation.role === "dps" ? gameData.value.dpsDefaults : gameData.value.supportDefaults;
  graduation.effective_tags = [...(defaults ?? [])];
  graduation.main_plan = { ...(DEFAULT_MAIN_PLAN[graduation.game] ?? {}) };
  if (graduation.game === "genshin" && graduation.role === "support") {
    Object.assign(graduation.main_plan, { sands: "energy_recharge", goblet: "elemental_mastery", circlet: "elemental_mastery" });
  }
  if (graduation.game === "hsr" && graduation.role === "support") {
    Object.assign(graduation.main_plan, { body: "effect_hit_rate", feet: "speed", planar_sphere: "hp_pct", link_rope: "energy_regen" });
  }
  graduation.base = {};
  graduation.panel_goals = {};
  graduation.hit_goals = {};
  for (const stat of trackedStats.value) {
    graduation.base[stat] = gameData.value.base?.[stat] ?? 0;
    graduation.hit_goals[stat] = gameData.value.hitTargets?.[stat] ?? 0;
    graduation.panel_goals[stat] = convertGoalValue(graduation.hit_goals[stat], stat, "hits", "panel", currentConversionConfig());
  }
  lastGoalEditKind.value = "hits";
  singleResult.value = null;
  averageResult.value = null;
}

function ensureGraduationFields() {
  for (const stat of trackedStats.value) {
    graduation.base[stat] ??= gameData.value.base?.[stat] ?? 0;
    graduation.hit_goals[stat] ??= gameData.value.hitTargets?.[stat] ?? 0;
    graduation.panel_goals[stat] ??= convertGoalValue(
      Number(graduation.hit_goals[stat] || 0),
      stat,
      "hits",
      "panel",
      currentConversionConfig(),
    );
  }
}

function currentConversionConfig() {
  return {
    game: graduation.game,
    base: graduation.base,
    mainPlan: graduation.main_plan,
  };
}

function syncGoal(stat, kind) {
  lastGoalEditKind.value = kind;
  const value = Number(kind === "panel" ? graduation.panel_goals[stat] : graduation.hit_goals[stat]) || 0;
  const otherKind = kind === "panel" ? "hits" : "panel";
  const converted = convertGoalValue(value, stat, kind, otherKind, currentConversionConfig());
  if (otherKind === "hits") graduation.hit_goals[stat] = converted;
  else graduation.panel_goals[stat] = converted;
  singleResult.value = null;
  averageResult.value = null;
}

function syncGoalsAfterBaseOrMain() {
  if (lastGoalEditKind.value === "panel") return;
  for (const stat of trackedStats.value) {
    graduation.panel_goals[stat] = convertGoalValue(Number(graduation.hit_goals[stat] || 0), stat, "hits", "panel", currentConversionConfig());
  }
  singleResult.value = null;
  averageResult.value = null;
}

function graduationPayload() {
  ensureGraduationFields();
  const effectiveTags = [...graduation.effective_tags];
  const stats = coreTrackedStats(graduation.game, graduation.role, effectiveTags);
  const sampleCount = Math.max(1, Number.parseInt(graduation.sample_count, 10) || 1);
  return {
    game: graduation.game,
    role: graduation.role,
    effectiveTags,
    stats,
    base: Object.fromEntries(stats.map((stat) => [stat, Number(graduation.base[stat] ?? 0)])),
    panelGoals: Object.fromEntries(stats.map((stat) => [stat, Math.max(0, Number(graduation.panel_goals[stat] ?? 0))])),
    hitGoals: Object.fromEntries(stats.map((stat) => [stat, Math.max(0, Number(graduation.hit_goals[stat] ?? 0))])),
    mainPlan: Object.fromEntries(slotList(graduation.game).map((slot) => [slot, graduation.main_plan[slot]])),
    maxRuns: Math.max(1, Number.parseInt(graduation.max_runs, 10) || 1),
    runsPerDay: Math.max(1, Number(graduation.runs_per_day) || 1),
    sampleCount,
    workerCount: Math.min(maxWorkerCount.value, sampleCount, Math.max(1, Number.parseInt(graduation.worker_count, 10) || 1)),
  };
}

async function runSingleSimulation() {
  await withBusy(async () => {
    const config = { ...graduationPayload(), seed: runtimeSeed("run") };
    singleResult.value = simulate(config);
    resetChartView();
  });
}

async function runAverageSimulation() {
  await withBusy(async () => {
    const config = { ...graduationPayload(), seed: runtimeSeed("average") };
    averageResult.value = null;
    averageProgress.done = 0;
    averageProgress.total = config.sampleCount;
    await runAverageInWorker(config);
  });
}

function stopAverageWorker() {
  for (const worker of averageWorkers) {
    worker.terminate();
  }
  averageWorkers = [];
}

function runAverageInWorker(config) {
  stopAverageWorker();
  return new Promise((resolve, reject) => {
    const workerCount = config.workerCount;
    const workerStates = Array.from({ length: workerCount }, () => ({ done: 0, sampleResults: [] }));
    let finishedWorkers = 0;
    let settled = false;

    const aggregate = () => {
      const done = workerStates.reduce((total, state) => total + state.done, 0);
      const sampleResults = workerStates.flatMap((state) => state.sampleResults);
      averageProgress.done = done;
      averageProgress.total = config.sampleCount;
      return summarizeAverage(config, sampleResults, done);
    };

    const finish = (callback, value) => {
      if (settled) return;
      settled = true;
      stopAverageWorker();
      callback(value);
    };

    for (let index = 0; index < workerCount; index += 1) {
      const startIndex = Math.floor((config.sampleCount * index) / workerCount);
      const endIndex = Math.floor((config.sampleCount * (index + 1)) / workerCount);
      const sampleCount = endIndex - startIndex;
      const worker = new Worker(new URL("./workers/averageWorker.js", import.meta.url), { type: "module" });
      averageWorkers.push(worker);

      worker.addEventListener("message", (event) => {
        const { type, done, sampleResults, message } = event.data ?? {};
        if (type === "progress" || type === "done") {
          workerStates[index] = {
            done: Number(done) || 0,
            sampleResults: Array.isArray(sampleResults) ? sampleResults : [],
          };
          averageResult.value = aggregate();
          if (type === "done") {
            finishedWorkers += 1;
            if (finishedWorkers === workerCount) finish(resolve);
          }
          return;
        }
        if (type === "error") {
          finish(reject, new Error(message || "平均统计计算失败"));
        }
      });

      worker.addEventListener("error", (event) => {
        finish(reject, new Error(event.message || "平均统计 Worker 运行失败"));
      });

      worker.postMessage({ config, startIndex, sampleCount });
    }
  });
}

onMounted(async () => {
  resetGraduation();
  await withBusy(async () => {
    meta.value = await getMeta();
    resetItemPools();
  });
});

onUnmounted(() => {
  stopAverageWorker();
});
</script>
