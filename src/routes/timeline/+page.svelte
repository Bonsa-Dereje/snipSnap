<script>
  import './timeline.css';

  // ── State ─────────────────────────────────────────────────
  let isPlaying = false;
  let currentTime = '00:04:15:12';
  let totalTime   = '00:12:44:00';
  let zoomLevel   = 55; // percent for slider

  const captions = [
    {
      start: '00:04:12',
      end:   '00:04:15',
      mood:  'CALM',
      text:  '"The silence in the room felt heavy, yet peaceful."',
      active: true,
    },
    {
      start: '00:04:16',
      end:   '00:04:18',
      mood:  'INTENSE',
      text:  '"Wait, did you hear that? I\'m sure someone\'s outside."',
      active: false,
    },
    {
      start: '00:04:19',
      end:   '00:04:22',
      mood:  'NEUTRAL',
      text:  '"I\'ll go check the kitchen. Stay here."',
      active: false,
    },
    {
      start: '00:04:23',
      end:   '00:04:28',
      mood:  'MELANCHOLY',
      text:  '"It doesn\'t matter anymore. Nothing ever does."',
      active: false,
    },
  ];

  const scenes = [
    { id: 'Scene 01_Intro',     time: '4s 12f', mood: 'Calm',    x: 0,   y: 0,   w: 160, connected: ['Scene 02_Confront'] },
    { id: 'Scene 02_Confront',  time: '2s 04f', mood: 'Intense', x: 200, y: 60,  w: 160, connected: ['Scene 03_Resoluti'] },
    { id: 'Scene 03_Resoluti',  time: '5s 15f', mood: 'Sad',     x: 400, y: 0,   w: 155, connected: [] },
  ];

  const takeAlts = [
    { id: 'TAKE_01', label: 'TAKE_01 (Active)', active: true,  thumb: 'forest' },
    { id: 'TAKE_02', label: 'TAKE_02',          active: false, thumb: 'city'   },
    { id: 'TAKE_03', label: 'TAKE_03',          active: false, thumb: 'desk'   },
  ];

  const moodClass = { CALM: 'mood-calm', INTENSE: 'mood-intense', NEUTRAL: 'mood-neutral', MELANCHOLY: 'mood-melancholy' };
  const sceneMoodClass = { Calm: 'scene-calm', Intense: 'scene-intense', Sad: 'scene-sad' };
</script>

<div class="tl-root">

  <!-- ══════════════════════════════════════════════════
       THREE-COLUMN BODY
       ══════════════════════════════════════════════════ -->
  <div class="tl-body">

    <!-- ── LEFT: SRT Caption Feed ─────────────────────── -->
    <aside class="tl-left">
      <div class="panel-header">
        <span class="panel-title">SRT CAPTION FEED</span>
        <button class="icon-btn-sm">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="12" cy="19" r="1"/>
          </svg>
        </button>
      </div>

      <div class="caption-list">
        {#each captions as cap}
          <div class="caption-item {cap.active ? 'caption-active' : ''}">
            <div class="caption-top">
              <span class="caption-time">{cap.start} – {cap.end}</span>
              <span class="caption-mood {moodClass[cap.mood]}">{cap.mood}</span>
            </div>
            <p class="caption-text">{cap.text}</p>
          </div>
        {/each}
      </div>
    </aside>

    <!-- ── CENTER: Preview + Node Graph ──────────────── -->
    <main class="tl-center">

      <!-- Timecode bar -->
      <div class="tc-bar">
        <span class="tc-code">{currentTime}</span>
        
      </div>

      <!-- Video preview -->
      <div class="preview-wrap">
        <div class="preview-screen">
          <!-- Forest scene SVG -->
          <svg class="preview-img" viewBox="0 0 540 305" xmlns="http://www.w3.org/2000/svg">
            <defs>
              <radialGradient id="fogGrad" cx="50%" cy="60%" r="60%">
                <stop offset="0%" stop-color="rgba(180,200,210,0.35)"/>
                <stop offset="100%" stop-color="rgba(80,110,130,0)"/>
              </radialGradient>
              <linearGradient id="skyGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#1a2a30"/>
                <stop offset="50%" stop-color="#253540"/>
                <stop offset="100%" stop-color="#1e4a38"/>
              </linearGradient>
              <linearGradient id="groundGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#1a2e1e"/>
                <stop offset="100%" stop-color="#0a1a0c"/>
              </linearGradient>
            </defs>
            <!-- Sky -->
            <rect width="540" height="305" fill="url(#skyGrad)"/>
            <!-- Distant fog -->
            <ellipse cx="270" cy="180" rx="250" ry="80" fill="url(#fogGrad)"/>
            <!-- Far trees -->
            <path d="M0 200 Q30 160 60 195 Q90 155 120 192 Q150 145 180 190 Q210 150 240 188 Q270 148 300 186 Q330 152 360 190 Q390 148 420 188 Q450 155 480 192 Q510 160 540 195 L540 305 L0 305Z" fill="rgba(20,45,28,0.6)"/>
            <!-- Mid trees -->
            <path d="M30 250 L30 185 L10 185 L45 130 L80 185 L60 185 L60 250Z" fill="rgba(15,40,22,0.9)"/>
            <path d="M100 250 L100 175 L78 175 L115 118 L152 175 L130 175 L130 250Z" fill="rgba(12,35,20,0.85)"/>
            <path d="M190 250 L190 190 L172 190 L204 140 L236 190 L218 190 L218 250Z" fill="rgba(18,42,24,0.8)"/>
            <path d="M310 250 L310 188 L292 188 L325 138 L358 188 L340 188 L340 250Z" fill="rgba(15,40,22,0.85)"/>
            <path d="M390 250 L390 178 L370 178 L408 122 L446 178 L426 178 L426 250Z" fill="rgba(12,36,20,0.9)"/>
            <path d="M470 250 L470 195 L454 195 L484 148 L514 195 L498 195 L498 250Z" fill="rgba(18,42,24,0.8)"/>
            <!-- Path -->
            <path d="M230 305 Q260 230 270 210 Q280 230 310 305Z" fill="rgba(80,110,90,0.25)"/>
            <path d="M240 305 Q265 235 270 215 Q275 235 300 305Z" fill="rgba(100,130,110,0.18)"/>
            <!-- Ground -->
            <rect x="0" y="240" width="540" height="65" fill="url(#groundGrad)" opacity="0.8"/>
            <!-- Figure silhouette -->
            <ellipse cx="270" cy="235" rx="7" ry="16" fill="rgba(8,15,10,0.92)"/>
            <ellipse cx="270" cy="218" rx="5" ry="6" fill="rgba(8,15,10,0.92)"/>
            <!-- Atmospheric haze -->
            <rect width="540" height="305" fill="rgba(150,190,200,0.07)"/>
          </svg>
        </div>

        <!-- Progress bar -->
        <div class="progress-bar">
          <div class="progress-fill" style="width: 34%;"></div>
          <div class="progress-head" style="left: 34%;"></div>
        </div>

        <!-- Playback controls -->
        <div class="playback-controls">
          <button class="pb-btn">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><polygon points="19,20 9,12 19,4"/><line x1="5" y1="4" x2="5" y2="20" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/></svg>
          </button>
          <button class="pb-btn">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 18 9 12 15 6"/></svg>
          </button>
          <button class="pb-btn pb-play" on:click={() => isPlaying = !isPlaying}>
            {#if isPlaying}
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
            {:else}
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><polygon points="5,3 19,12 5,21"/></svg>
            {/if}
          </button>
          <button class="pb-btn">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/><polyline points="15 18 21 12 15 6"/></svg>
          </button>
          <button class="pb-btn">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><polygon points="5,4 15,12 5,20"/><line x1="19" y1="4" x2="19" y2="20" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/></svg>
          </button>
        </div>
      </div>

      <!-- Node Scene Graph -->
      <div class="node-graph-wrap">
        <div class="panel-header" style="padding: 10px 14px 8px;">
          <span class="panel-title">NODE SCENE GRAPH</span>
        </div>
        <div class="node-canvas">
          <!-- Connector lines -->
          <svg class="node-svg" viewBox="0 0 580 130" preserveAspectRatio="none">
            <!-- Scene01 → Scene02 -->
            <path d="M160 38 C190 38, 195 75, 215 75" stroke="rgba(140,150,160,0.4)" stroke-width="1.5" fill="none" stroke-dasharray="4 3"/>
            <!-- Scene02 → Scene03 -->
            <path d="M375 75 C395 75, 400 42, 415 42" stroke="rgba(140,150,160,0.4)" stroke-width="1.5" fill="none" stroke-dasharray="4 3"/>
          </svg>

          <!-- Scene nodes -->
          <div class="scene-node" style="left:12px; top:14px;">
            <div class="scene-node-header">
              <span class="scene-node-id">Scene 01_Intro</span>
              <button class="icon-btn-sm">
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M12 2v3M12 19v3M4.22 4.22l2.12 2.12M17.66 17.66l2.12 2.12M2 12h3M19 12h3M4.22 19.78l2.12-2.12M17.66 6.34l2.12-2.12"/></svg>
              </button>
            </div>
            <div class="scene-node-meta">4s 12f <span class="scene-tag scene-calm">Calm</span></div>
          </div>

          <div class="scene-node scene-node-active" style="left:212px; top:50px;">
            <div class="scene-node-header">
              <span class="scene-node-id">Scene 02_Confront</span>
              <button class="icon-btn-sm" style="color:var(--accent)">
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 3 21 3 21 9"/><polyline points="9 21 3 21 3 15"/><line x1="21" y1="3" x2="14" y2="10"/><line x1="3" y1="21" x2="10" y2="14"/></svg>
              </button>
            </div>
            <div class="scene-node-meta">2s 04f <span class="scene-tag scene-intense">Intense</span></div>
          </div>

          <div class="scene-node" style="left:412px; top:14px;">
            <div class="scene-node-header">
              <span class="scene-node-id">Scene 03_Resoluti…</span>
            </div>
            <div class="scene-node-meta">5s 15f <span class="scene-tag scene-sad">Sad</span></div>
          </div>
        </div>
      </div>

    </main>

    <!-- ── RIGHT: AI Panel ────────────────────────────── -->
    <aside class="tl-right">

      <!-- Emotional Moments -->
      <div class="panel-header">
        <span class="panel-title">EMOTIONAL MOMENTS (AI)</span>
        <button class="icon-btn-sm">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        </button>
      </div>

      <div class="emo-feature">
        <div class="emo-thumb emo-thumb-eye">
          <svg width="100%" height="100%" viewBox="0 0 200 120" xmlns="http://www.w3.org/2000/svg">
            <defs>
              <radialGradient id="irisGrad" cx="50%" cy="50%" r="50%">
                <stop offset="0%" stop-color="#c8702a"/>
                <stop offset="40%" stop-color="#8b4513"/>
                <stop offset="80%" stop-color="#5c2800"/>
                <stop offset="100%" stop-color="#2a1000"/>
              </radialGradient>
              <radialGradient id="bgGrad" cx="50%" cy="50%" r="60%">
                <stop offset="0%" stop-color="#4a2000"/>
                <stop offset="100%" stop-color="#0a0400"/>
              </radialGradient>
            </defs>
            <rect width="200" height="120" fill="url(#bgGrad)"/>
            <ellipse cx="100" cy="60" rx="80" ry="55" fill="rgba(20,8,0,0.3)"/>
            <circle cx="100" cy="60" r="44" fill="url(#irisGrad)"/>
            <!-- Iris texture lines -->
            {#each Array(16) as _, i}
              <line
                x1="100" y1="60"
                x2={100 + 44 * Math.cos(i * Math.PI / 8)}
                y2={60 + 44 * Math.sin(i * Math.PI / 8)}
                stroke="rgba(0,0,0,0.3)" stroke-width="1"
              />
            {/each}
            <!-- Pupil -->
            <circle cx="100" cy="60" r="18" fill="#080400"/>
            <!-- Specular highlight -->
            <ellipse cx="110" cy="50" rx="8" ry="6" fill="rgba(255,255,255,0.65)" style="filter:blur(2px)"/>
            <ellipse cx="89" cy="68" rx="4" ry="3" fill="rgba(255,255,255,0.25)" style="filter:blur(1px)"/>
          </svg>
          <span class="thumb-tc">04:12</span>
          <button class="thumb-pin">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/><circle cx="12" cy="10" r="3"/></svg>
          </button>
        </div>
        <div class="emo-info">
          <div class="emo-title">The Realization</div>
          <div class="emo-meta">Confidence: 98% • Peak Melancholy</div>
        </div>
      </div>

      <!-- Take Alternatives -->
      <div class="panel-header" style="margin-top:8px; border-top: 1px solid var(--border-subtle);">
        <span class="panel-title">TAKE ALTERNATIVES</span>
        <button class="icon-btn-sm">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
        </button>
      </div>

      <div class="takes-grid">
        {#each takeAlts as t}
          <div class="take-card {t.active ? 'take-active' : ''}">
            <div class="take-thumb">
              {#if t.thumb === 'forest'}
                <div class="tthumb-forest" style="width:100%;height:100%;"></div>
              {:else if t.thumb === 'city'}
                <div class="tthumb-city" style="width:100%;height:100%;"></div>
              {:else if t.thumb === 'desk'}
                <div class="tthumb-desk" style="width:100%;height:100%;"></div>
              {/if}
            </div>
            <div class="take-label">{t.label}</div>
          </div>
        {/each}
      </div>

    </aside>
  </div>

  <!-- ══════════════════════════════════════════════════
       TIMELINE BOTTOM
       ══════════════════════════════════════════════════ -->
  <div class="tl-bottom">

    <!-- Timeline toolbar -->
    <div class="tl-toolbar">
      <div class="tl-tools-left">
        <button class="tl-tool-btn" title="Cut">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="6" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><line x1="20" y1="4" x2="8.12" y2="15.88"/><line x1="14.47" y1="14.48" x2="20" y2="20"/><line x1="8.12" y1="8.12" x2="12" y2="12"/></svg>
        </button>
        <button class="tl-tool-btn" title="Copy">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
        </button>
        <button class="tl-tool-btn" title="Trim">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/></svg>
        </button>
        <button class="tl-tool-btn" title="Ripple">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 18 9 12 15 6"/><polyline points="9 18 3 12 9 6"/></svg>
        </button>
      </div>

      <div class="tl-zoom">
        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
        <input type="range" min="10" max="100" bind:value={zoomLevel} class="zoom-slider"/>
        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="11" y1="8" x2="11" y2="14"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
      </div>

      <div class="tl-timecode">
        <span class="tc-current">{currentTime}</span>
        <span class="tc-sep">/</span>
        <span class="tc-total">{totalTime}</span>
      </div>
    </div>

    <!-- Track headers + ruler -->
    <div class="tl-tracks-wrap">
      <!-- Ruler -->
      <div class="tl-ruler-row">
        <div class="track-header-spacer"></div>
        <div class="tl-ruler">
          {#each ['00:04:00', '00:04:10', '00:04:20', '00:04:30', '00:04:40'] as mark}
            <span class="ruler-mark">{mark}</span>
          {/each}
          <!-- Playhead -->
          <div class="playhead" style="left: 34%;"></div>
        </div>
      </div>

      <!-- V1 track -->
      <div class="tl-track-row">
        <div class="track-header">
          <span class="track-label">V1</span>
          <button class="track-vis-btn">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
          </button>
        </div>
        <div class="track-lane">
          <div class="clip-block clip-v1-a" style="left:0%; width:35%;">intro_park_wide.mp4</div>
          <div class="clip-block clip-v1-b" style="left:35%; width:22%;">confrontation_CU.mp4</div>
        </div>
      </div>

      <!-- A1 track -->
      <div class="tl-track-row">
        <div class="track-header">
          <span class="track-label">A1</span>
          <button class="track-vis-btn">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/></svg>
          </button>
        </div>
        <div class="track-lane">
          <div class="clip-block clip-audio" style="left:0%; width:57%;">
            <!-- Waveform bars -->
            <div class="waveform">
              {#each Array(40) as _, i}
                <div class="wave-bar" style="height: {20 + Math.sin(i * 0.7) * 12 + Math.random() * 8}px;"></div>
              {/each}
            </div>
          </div>
        </div>
      </div>

      <!-- EMO track -->
      <div class="tl-track-row">
        <div class="track-header">
          <span class="track-label">EMO</span>
          <button class="track-vis-btn">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><line x1="9" y1="9" x2="9.01" y2="9"/><line x1="15" y1="9" x2="15.01" y2="9"/></svg>
          </button>
        </div>
        <div class="track-lane">
          <div class="clip-block clip-emo-calm"   style="left:0%; width:25%;"></div>
          <div class="clip-block clip-emo-intense" style="left:25%; width:12%;"></div>
          <div class="clip-block clip-emo-neutral" style="left:37%; width:10%;"></div>
          <div class="clip-block clip-emo-mel"     style="left:47%; width:12%;"></div>
        </div>
      </div>
    </div>

  </div>

</div>