<script>
  import { onMount } from 'svelte';

  // ── State ─────────────────────────────────────────────────
  let activeMenu = 'Timeline';
  let activeSidebar = 'TIMELINE';
  let activeCaption = 1;
  let activeTake = 'TAKE_01';
  let zoomLevel = 55;

  const menuItems = ['Timeline', 'File', 'Edit', 'Sequence', 'Clip', 'View'];

  const sidebarItems = [
    { id: 'PROJECT',   label: 'PROJECT',   icon: 'project' },
    { id: 'TIMELINE',  label: 'TIMELINE',  icon: 'timeline' },
    { id: 'MEDIA',     label: 'MEDIA',     icon: 'media' },
    { id: 'NODES',     label: 'NODES',     icon: 'nodes' },
    { id: 'INSPECTOR', label: 'INSPECTOR', icon: 'inspector' },
  ];

  const captions = [
    {
      id: 0, emotion: 'calm',
      start: '00:04:12', end: '00:04:15',
      tag: 'CALM',
      text: '"The silence in the room felt heavy, yet peaceful."'
    },
    {
      id: 1, emotion: 'intense',
      start: '00:04:16', end: '00:04:18',
      tag: 'INTENSE',
      text: '"Wait, did you hear that? I\'m sure someone\'s outside."'
    },
    {
      id: 2, emotion: 'neutral',
      start: '00:04:19', end: '00:04:22',
      tag: 'NEUTRAL',
      text: '"I\'ll go check the kitchen. Stay here."'
    },
    {
      id: 3, emotion: 'melancholy',
      start: '00:04:23', end: '00:04:28',
      tag: 'MELANCHOLY',
      text: '"It doesn\'t matter anymore. Nothing ever does."'
    },
  ];

  const takes = [
    { id: 'TAKE_01', cls: 'take-01', active: true },
    { id: 'TAKE_02', cls: 'take-02', active: false },
    { id: 'TAKE_03', cls: 'take-03', active: false },
  ];

  // Waveform bars — random-seeded heights for realistic look
  const waveBars = Array.from({ length: 80 }, (_, i) => {
    const t = i / 80;
    const h = Math.abs(Math.sin(t * 18 + 0.4) * 0.6 + Math.sin(t * 33 + 1.2) * 0.3 + Math.sin(t * 7) * 0.1) * 0.85 + 0.12;
    return Math.min(h, 0.97);
  });

  // Node graph SVG connector path helper
  // node-01 center-right → node-02 center-left → node-03 center-left
  const nodePath1 = 'M 130 12 C 155 12, 155 26, 170 26';
  const nodePath2 = 'M 300 26 C 325 26, 325 12, 340 12';
</script>

<div class="app-shell">

  <!-- ═══════════════════════════════════════════════════════
       TOP MENU BAR
       ═══════════════════════════════════════════════════════ -->
  <header class="menu-bar">
    

    <nav class="menu-items">
      {#each menuItems as item}
        <button
          class="menu-item {activeMenu === item ? 'active' : ''}"
          on:click={() => activeMenu = item}
        >{item}</button>
      {/each}
    </nav>

    <div class="menu-bar-right">
      <div class="search-bar">
        <!-- Search icon -->
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input type="text" placeholder="Search assets..." />
      </div>

      <!-- Settings -->
      <button class="icon-btn" title="Settings">
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>
      <!-- Help -->
      <button class="icon-btn" title="Help">
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/>
        </svg>
      </button>
      <!-- User -->
      <button class="icon-btn" title="Account">
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>
        </svg>
      </button>
    </div>
  </header>

  <!-- ═══════════════════════════════════════════════════════
       LEFT ICON SIDEBAR
       ═══════════════════════════════════════════════════════ -->
  <aside class="icon-sidebar">
    {#each sidebarItems as item}
      <button
        class="sidebar-item {activeSidebar === item.id ? 'active' : ''}"
        on:click={() => activeSidebar = item.id}
        title={item.label}
      >
        {#if item.icon === 'project'}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/>
          </svg>
        {:else if item.icon === 'timeline'}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="8" width="18" height="3" rx="1"/><rect x="3" y="13" width="13" height="3" rx="1"/><line x1="3" y1="5" x2="21" y2="5"/>
          </svg>
        {:else if item.icon === 'media'}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="2" width="20" height="20" rx="2.18"/><line x1="7" y1="2" x2="7" y2="22"/><line x1="17" y1="2" x2="17" y2="22"/><line x1="2" y1="12" x2="22" y2="12"/><line x1="2" y1="7" x2="7" y2="7"/><line x1="2" y1="17" x2="7" y2="17"/><line x1="17" y1="17" x2="22" y2="17"/><line x1="17" y1="7" x2="22" y2="7"/>
          </svg>
        {:else if item.icon === 'nodes'}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="5" cy="12" r="2"/><circle cx="19" cy="5" r="2"/><circle cx="19" cy="19" r="2"/><line x1="7" y1="12" x2="17" y2="6"/><line x1="7" y1="12" x2="17" y2="18"/>
          </svg>
        {:else if item.icon === 'inspector'}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="4" y1="6" x2="20" y2="6"/><line x1="4" y1="12" x2="14" y2="12"/><line x1="4" y1="18" x2="10" y2="18"/>
          </svg>
        {/if}
        <span class="sidebar-label">{item.label}</span>
      </button>
    {/each}

    <div class="sidebar-spacer"></div>

    <div class="sidebar-bottom">
      <!-- Sync -->
      <button class="sidebar-item sync-item" title="Sync">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="23 4 23 10 17 10"/><polyline points="1 20 1 14 7 14"/>
          <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
        </svg>
        <span class="sidebar-label">SYNC</span>
      </button>
    </div>
  </aside>

  <!-- ═══════════════════════════════════════════════════════
       MAIN CONTENT
       ═══════════════════════════════════════════════════════ -->
  <main class="main-content">

    <!-- ─── LEFT: SRT Caption Feed ─────────────────────────── -->
    <section class="caption-panel">
      <div class="panel-header">
        <span class="panel-title">SRT Caption Feed</span>
        <button class="icon-btn" title="Options">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="12" cy="19" r="1"/>
          </svg>
        </button>
      </div>

      <div class="captions-list">
        {#each captions as cap}
          <div
            class="caption-entry {cap.emotion} {activeCaption === cap.id ? 'active' : ''}"
            on:click={() => activeCaption = cap.id}
          >
            <div class="caption-meta">
              <span class="caption-time">{cap.start} – {cap.end}</span>
              <span class="caption-tag">{cap.tag}</span>
            </div>
            <p class="caption-text">{cap.text}</p>
          </div>
        {/each}
      </div>
    </section>

    <!-- ─── CENTER: Preview + Node Graph ──────────────────── -->
    <section class="center-panel">

      <!-- Preview -->
      <div class="preview-wrap">
        <div class="preview-topbar">
          <span class="preview-timecode">00:04:15:12</span>
          <div class="live-badge">
            <span class="live-dot"></span>
            LIVE PREVIEW
          </div>
        </div>

        <div class="preview-canvas">
          <!-- Cinematic forest scene placeholder -->
          <div class="preview-scene" aria-label="Preview: forest path scene"></div>
        </div>

        <div class="preview-progress">
          <div class="preview-progress-fill"></div>
        </div>

        <div class="preview-controls">
          <!-- Skip to start -->
          <button class="ctrl-btn" title="Go to start">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polygon points="19 20 9 12 19 4 19 20"/><line x1="5" y1="19" x2="5" y2="5"/>
            </svg>
          </button>
          <!-- Step back -->
          <button class="ctrl-btn" title="Previous frame">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="15 18 9 12 15 6"/>
            </svg>
          </button>
          <!-- Play -->
          <button class="ctrl-btn play-btn" title="Play / Pause">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="0"/>
              <polygon points="10 8 16 12 10 16 10 8"/>
            </svg>
          </button>
          <!-- Step forward -->
          <button class="ctrl-btn" title="Next frame">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"/>
            </svg>
          </button>
          <!-- Skip to end -->
          <button class="ctrl-btn" title="Go to end">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polygon points="5 4 15 12 5 20 5 4"/><line x1="19" y1="5" x2="19" y2="19"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Node Scene Graph -->
      <div class="node-graph-section">
        <div class="node-graph-title">Node Scene Graph</div>
        <div class="node-graph-canvas">

          <!-- SVG connector lines -->
          <svg class="node-connectors" viewBox="0 0 480 96" preserveAspectRatio="none">
            <!-- Line: node01 → node02 -->
            <path d={nodePath1} fill="none" stroke="rgba(255,255,255,0.15)" stroke-width="1.5" stroke-dasharray="5,4"/>
            <!-- Line: node02 → node03 -->
            <path d={nodePath2} fill="none" stroke="rgba(255,255,255,0.15)" stroke-width="1.5" stroke-dasharray="5,4"/>
          </svg>

          <!-- Node 01 -->
          <div class="scene-node node-01">
            <div class="node-name">
              Scene 01_Intro
              <button class="node-icon-btn" title="Settings">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33..."/></svg>
              </button>
            </div>
            <div class="node-meta">
              <span>4s 12f</span>
              <span class="node-meta-dot">•</span>
              <span style="color: var(--tag-calm)">Calm</span>
            </div>
          </div>

          <!-- Node 02 -->
          <div class="scene-node node-02">
            <div class="node-name">
              Scene 02_Confront
              <button class="node-icon-btn" title="Detach">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 3 21 3 21 9"/><polyline points="9 21 3 21 3 15"/><line x1="21" y1="3" x2="14" y2="10"/><line x1="3" y1="21" x2="10" y2="14"/></svg>
              </button>
            </div>
            <div class="node-meta">
              <span>2s 04f</span>
              <span class="node-meta-dot">•</span>
              <span style="color: var(--tag-intense)">Intense</span>
            </div>
          </div>

          <!-- Node 03 -->
          <div class="scene-node node-03">
            <div class="node-name">Scene 03_Resoluti…</div>
            <div class="node-meta">
              <span>5s 15f</span>
              <span class="node-meta-dot">•</span>
              <span style="color: var(--tag-melancholy)">Sad</span>
            </div>
          </div>

        </div>
      </div>

    </section>

    <!-- ─── RIGHT: Emotional Moments + Takes ──────────────── -->
    <aside class="right-panel">

      <!-- Emotional Moments header -->
      <div class="panel-header">
        <span class="panel-title">Emotional Moments (AI)</span>
        <button class="icon-btn" title="Info">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        </button>
      </div>

      <!-- Featured thumbnail — eye close-up -->
      <div class="right-panel-section">
        <div class="emotion-thumb featured">
          <!-- Stylised eye placeholder (CSS-drawn) -->
          <div style="
            width:100%; height:160px;
            background: radial-gradient(ellipse 55% 70% at 50% 50%, #8b4513 0%, #5c2800 30%, #1a0800 60%, #000 100%);
            position:relative; overflow:hidden;
          ">
            <!-- Iris ring -->
            <div style="
              position:absolute; top:50%; left:50%;
              transform: translate(-50%,-50%);
              width:80px; height:80px;
              border-radius:50%;
              background: conic-gradient(from 0deg, #c47a2a, #8b4513, #d4882e, #6b3410, #c47a2a);
              box-shadow: 0 0 20px rgba(180,100,30,0.4);
            "></div>
            <!-- Pupil -->
            <div style="
              position:absolute; top:50%; left:50%;
              transform: translate(-50%,-50%);
              width:28px; height:28px;
              border-radius:50%;
              background: #000;
            "></div>
            <!-- Glint -->
            <div style="
              position:absolute; top:calc(50% - 14px); left:calc(50% + 8px);
              width:10px; height:10px;
              border-radius:50%;
              background: rgba(255,255,255,0.7);
              filter: blur(2px);
            "></div>
          </div>
          <span class="thumb-timecode">04:12</span>
        </div>

        <div class="emotion-info">
          <div class="emotion-title">
            The Realization
            <span class="emotion-badge">
              <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
            </span>
          </div>
          <div class="emotion-meta">Confidence: 98% &bull; Peak Melancholy</div>
        </div>
      </div>

      <!-- Second thumbnail — dark corridor -->
      <div class="emotion-thumb" style="flex-shrink:0;">
        <div style="
          width:100%; height:100px;
          background: linear-gradient(135deg, #0a0f14 0%, #0d1c28 40%, #1a2840 70%, #0a1520 100%);
          position:relative; overflow:hidden;
        ">
          <!-- Corridor light streak -->
          <div style="
            position:absolute; top:0; right:30%; bottom:0; width:2px;
            background: linear-gradient(to bottom, transparent, rgba(100,160,255,0.3), transparent);
          "></div>
          <div style="
            position:absolute; top:20%; left:60%; right:0; bottom:20%;
            background: linear-gradient(to right, transparent, rgba(60,120,200,0.08));
          "></div>
        </div>
      </div>

      <!-- Take Alternatives -->
      <div class="takes-header">
        <span class="panel-title">Take Alternatives</span>
        <button class="icon-btn" title="Copy takes">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
        </button>
      </div>

      <div class="takes-grid">
        <!-- Take 01 — forest path -->
        <div
          class="take-item {activeTake === 'TAKE_01' ? 'active' : ''}"
          on:click={() => activeTake = 'TAKE_01'}
        >
          <div class="take-img-wrap">
            <div class="take-placeholder take-01">
              <!-- Mini forest silhouette -->
              <svg width="60" height="40" viewBox="0 0 60 40" fill="none">
                <rect width="60" height="40" fill="transparent"/>
                <path d="M10 38 L10 22 L2 22 L15 8 L28 22 L20 22 L20 38Z" fill="rgba(20,50,30,0.8)"/>
                <path d="M30 38 L30 20 L22 20 L35 5 L48 20 L40 20 L40 38Z" fill="rgba(15,40,25,0.7)"/>
                <path d="M48 38 L48 24 L42 24 L51 12 L60 24 L54 24 L54 38Z" fill="rgba(20,45,28,0.75)"/>
                <line x1="0" y1="38" x2="60" y2="38" stroke="rgba(30,60,40,0.5)" stroke-width="0.5"/>
              </svg>
            </div>
          </div>
          <div class="take-label">TAKE_01 (Active)</div>
        </div>

        <!-- Take 02 — city skyline -->
        <div
          class="take-item {activeTake === 'TAKE_02' ? 'active' : ''}"
          on:click={() => activeTake = 'TAKE_02'}
        >
          <div class="take-img-wrap">
            <div class="take-placeholder take-02">
              <svg width="60" height="40" viewBox="0 0 60 40" fill="none">
                <defs>
                  <linearGradient id="sky2" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stop-color="#1a0a00"/>
                    <stop offset="60%" stop-color="#4a1a00"/>
                    <stop offset="100%" stop-color="#8a3000"/>
                  </linearGradient>
                </defs>
                <rect width="60" height="40" fill="url(#sky2)"/>
                <rect x="5"  y="20" width="6"  height="20" fill="rgba(10,10,20,0.9)"/>
                <rect x="14" y="15" width="5"  height="25" fill="rgba(15,12,25,0.9)"/>
                <rect x="22" y="22" width="8"  height="18" fill="rgba(10,10,20,0.85)"/>
                <rect x="33" y="12" width="5"  height="28" fill="rgba(12,10,22,0.9)"/>
                <rect x="41" y="18" width="9"  height="22" fill="rgba(10,10,20,0.85)"/>
                <rect x="52" y="24" width="5"  height="16" fill="rgba(15,10,22,0.8)"/>
              </svg>
            </div>
          </div>
          <div class="take-label">TAKE_02</div>
        </div>

        <!-- Take 03 — figure at desk -->
        <div
          class="take-item {activeTake === 'TAKE_03' ? 'active' : ''}"
          on:click={() => activeTake = 'TAKE_03'}
        >
          <div class="take-img-wrap">
            <div class="take-placeholder take-03">
              <svg width="60" height="40" viewBox="0 0 60 40" fill="none">
                <rect width="60" height="40" fill="rgba(8,8,18,1)"/>
                <!-- Figure outline -->
                <ellipse cx="30" cy="20" rx="6" ry="7" fill="rgba(20,20,35,0.9)"/>
                <ellipse cx="30" cy="11" rx="4" ry="4" fill="rgba(25,25,40,0.8)"/>
                <!-- Desk surface -->
                <rect x="10" y="30" width="40" height="2" fill="rgba(30,30,50,0.6)" rx="1"/>
                <!-- Ambient glow -->
                <ellipse cx="30" cy="28" rx="12" ry="4" fill="rgba(40,40,80,0.2)"/>
              </svg>
            </div>
          </div>
          <div class="take-label">TAKE_03</div>
        </div>
      </div>

    </aside>

    <!-- ─── BOTTOM: Timeline (spans full width) ─────────────── -->
    <div class="timeline-area">

      <!-- Toolbar -->
      <div class="timeline-toolbar">
        <div class="tl-tools">
          <!-- Cut -->
          <button class="tl-tool-btn" title="Cut">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="6" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><line x1="20" y1="4" x2="8.12" y2="15.88"/><line x1="14.47" y1="14.48" x2="20" y2="20"/><line x1="8.12" y1="8.12" x2="12" y2="12"/></svg>
          </button>
          <!-- Copy -->
          <button class="tl-tool-btn" title="Copy">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
          </button>
          <!-- Pen / Edit -->
          <button class="tl-tool-btn" title="Edit">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
          </button>
          <!-- Fit -->
          <button class="tl-tool-btn" title="Fit to window">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 3 21 3 21 9"/><polyline points="9 21 3 21 3 15"/><line x1="21" y1="3" x2="14" y2="10"/><line x1="3" y1="21" x2="10" y2="14"/></svg>
          </button>
        </div>

        <!-- Zoom -->
        <div class="tl-zoom-wrap">
          <svg class="tl-zoom-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
          <input
            class="tl-zoom-slider"
            type="range" min="10" max="100"
            bind:value={zoomLevel}
          />
          <svg class="tl-zoom-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="11" y1="8" x2="11" y2="14"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
        </div>

        <!-- Timecode readout -->
        <div class="tl-timecode-right">
          <span>00:04:15:12</span> / 00:12:44:00
        </div>
      </div>

      <!-- Tracks -->
      <div class="timeline-tracks">

        <!-- Playhead -->
        <div class="playhead"></div>

        <!-- Time ruler -->
        <div class="time-ruler">
          <span class="ruler-mark">00:04:00</span>
          <span class="ruler-mark">00:04:10</span>
          <span class="ruler-mark">00:04:20</span>
          <span class="ruler-mark">00:04:30</span>
          <span class="ruler-mark">00:04:40</span>
        </div>

        <!-- V1 Video track -->
        <div class="track-row tall">
          <div class="track-label">
            <span class="track-name">V1</span>
            <button class="track-vis-btn" title="Toggle visibility">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
            </button>
          </div>
          <div class="track-content">
            <div class="clip-block v1-a">intro_park_wide.mp4</div>
            <div class="clip-block v1-b">confrontation_CU.mp4</div>
          </div>
        </div>

        <!-- A1 Audio track -->
        <div class="track-row">
          <div class="track-label">
            <span class="track-name">A1</span>
            <button class="track-vis-btn" title="Toggle audio">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"/></svg>
            </button>
          </div>
          <div class="audio-track-content">
            {#each waveBars as h}
              <div class="waveform-bar" style="height: {Math.round(h * 100)}%"></div>
            {/each}
          </div>
        </div>

        <!-- EMO emotion track -->
        <div class="track-row">
          <div class="track-label">
            <span class="track-name">EMO</span>
            <button class="track-vis-btn" title="Toggle emotion layer">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><line x1="9" y1="9" x2="9.01" y2="9"/><line x1="15" y1="9" x2="15.01" y2="9"/></svg>
            </button>
          </div>
          <div class="track-content" style="padding: 4px 0;">
            <div class="emo-track-content">
              <div class="emo-segment emo-calm"></div>
              <div class="emo-segment emo-intense"></div>
              <div class="emo-segment emo-neutral"></div>
              <div class="emo-segment emo-melancholy"></div>
            </div>
          </div>
        </div>

      </div>
    </div>

  </main>

  <!-- ── Export button (absolute, bottom-left like reference) ── -->
  <button class="export-btn" style="
    position: fixed;
    bottom: 0;
    left: 0;
    width: var(--sidebar-w);
    justify-content: center;
    border-radius: 0;
    padding: 10px 4px;
    font-size: 10px;
    letter-spacing: 0.12em;
    z-index: 200;
  ">
    EXPORT
  </button>

</div>

<style>
  /* Component-scoped overrides / imports */
  @import './homePage.css';
</style>