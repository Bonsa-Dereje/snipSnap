<script>
import { goto } from '$app/navigation';

// ── State ─────────────────────────────────────────────────
let viewMode = 'list';
let sortBy   = 'Last Modified';
let searchQuery = '';

// Modal state
let showModal = false;
let isDragOver = false;
let uploadedFile = null;
let fileError = '';

const sortOptions = ['Last Modified', 'Name', 'Duration', 'Created'];

const projects = [
  {
    id: 1,
    title: 'The Silence',
    path: '/Projects/The_Silence',
    updated: '2 hours ago',
    resolution: '1920x1080',
    fps: '24fps',
    duration: '04:15',
    tags: ['Drama', 'Short Film'],
    tagColors: { Drama: 'blue', 'Short Film': 'green' },
    thumb: 'forest',
  },
  {
    id: 2,
    title: 'Cityscape',
    path: '/Projects/Cityscape',
    updated: 'yesterday',
    resolution: '1920x1080',
    fps: '24fps',
    duration: '02:31',
    tags: ['Commercial', 'Timelapse'],
    tagColors: { Commercial: 'teal', Timelapse: 'purple' },
    thumb: 'city',
  },
  {
    id: 3,
    title: 'The Realization',
    path: '/Projects/The_Realization',
    updated: '3 days ago',
    resolution: '1920x1080',
    fps: '24fps',
    duration: '01:08',
    tags: ['AI Short', 'Experimental'],
    tagColors: { 'AI Short': 'orange', Experimental: 'red' },
    thumb: 'eye',
  },
  {
    id: 4,
    title: 'Chase Sequence',
    path: '/Projects/Chase_Sequence',
    updated: '1 week ago',
    resolution: '1920x1080',
    fps: '24fps',
    duration: '03:47',
    tags: ['Action', 'Thriller'],
    tagColors: { Action: 'red', Thriller: 'purple' },
    thumb: 'dark',
  },
  {
    id: 5,
    title: 'The Negotiation',
    path: '/Projects/The_Negotiation',
    updated: '2 weeks ago',
    resolution: '1920x1080',
    fps: '24fps',
    duration: '02:05',
    tags: ['Drama', 'Dialogue'],
    tagColors: { Drama: 'blue', Dialogue: 'teal' },
    thumb: 'desk',
  },
  {
    id: 6,
    title: 'Nature Escape',
    path: '/Projects/Nature_Escape',
    updated: '2 weeks ago',
    resolution: '1920x1080',
    fps: '24fps',
    duration: '05:12',
    tags: ['Documentary', 'Nature'],
    tagColors: { Documentary: 'green', Nature: 'teal' },
    thumb: 'mountain',
  },
];

$: filtered = projects.filter(p =>
  p.title.toLowerCase().includes(searchQuery.toLowerCase())
);

const tagColorMap = {
  blue:   { bg: 'rgba(26,114,232,0.10)', color: '#1a72e8' },
  green:  { bg: 'rgba(26,153,85,0.10)',  color: '#1a9955' },
  teal:   { bg: 'rgba(11,143,143,0.10)', color: '#0b8f8f' },
  purple: { bg: 'rgba(124,82,200,0.10)', color: '#7c52c8' },
  orange: { bg: 'rgba(224,92,16,0.10)',  color: '#e05c10' },
  red:    { bg: 'rgba(217,48,37,0.10)',  color: '#d93025' },
};

// ── Modal helpers ──────────────────────────────────────────
function openModal() {
  showModal = true;
  uploadedFile = null;
  fileError = '';
  isDragOver = false;
}

function closeModal() {
  showModal = false;
  uploadedFile = null;
  fileError = '';
  isDragOver = false;
}

function validateAndSetFile(file) {
  fileError = '';
  if (!file) return;
  if (!file.name.toLowerCase().endsWith('.srt')) {
    fileError = 'Only .srt subtitle files are accepted.';
    uploadedFile = null;
    return;
  }
  uploadedFile = file;
}

function handleDrop(e) {
  e.preventDefault();
  isDragOver = false;
  const file = e.dataTransfer.files[0];
  validateAndSetFile(file);
}

function handleDragOver(e) {
  e.preventDefault();
  isDragOver = true;
}

function handleDragLeave() {
  isDragOver = false;
}

function handleFileInput(e) {
  validateAndSetFile(e.target.files[0]);
}

function handleCreate() {
  if (!uploadedFile) return;
  closeModal();
  goto('/timeline');
}

function handleKeydown(e) {
  if (e.key === 'Escape') closeModal();
}

function formatBytes(bytes) {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="projects-page">

  <!-- ── Header ─────────────────────────────────────────────── -->
  <div class="projects-header">
    <div class="header-left">
      <h1 class="page-title">Projects</h1>
      <p class="page-subtitle">Manage and organize your video editing projects.</p>
    </div>
    <button class="create-btn" on:click={openModal}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
      </svg>
      Create New Project
    </button>
  </div>

  <!-- ── Toolbar ────────────────────────────────────────────── -->
  <div class="projects-toolbar">
    <div class="toolbar-left">
      <span class="section-label">Recent Projects</span>
    </div>
    <div class="toolbar-right">
      <!-- Search -->
      <div class="toolbar-search">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input type="text" placeholder="Filter projects..." bind:value={searchQuery} />
      </div>

      <!-- Sort -->
      <div class="sort-wrap">
        <span class="sort-label">Sort by:</span>
        <select class="sort-select" bind:value={sortBy}>
          {#each sortOptions as opt}
            <option value={opt}>{opt}</option>
          {/each}
        </select>
      </div>

      <!-- View toggle -->
      <div class="view-toggle">
        <button
          class="view-btn {viewMode === 'grid' ? 'active' : ''}"
          title="Grid view"
          on:click={() => viewMode = 'grid'}
        >
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/>
            <rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/>
          </svg>
        </button>
        <button
          class="view-btn {viewMode === 'list' ? 'active' : ''}"
          title="List view"
          on:click={() => viewMode = 'list'}
        >
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/>
            <line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/>
          </svg>
        </button>
      </div>
    </div>
  </div>

  <!-- ── Project List ───────────────────────────────────────── -->
  <div class="projects-list-wrap">
    {#if viewMode === 'list'}
      <div class="projects-list">
        {#each filtered as project (project.id)}
          <div class="project-row">

            <!-- Thumbnail -->
            <div class="project-thumb">
              {#if project.thumb === 'forest'}
                <div class="thumb-bg thumb-forest">
                  <svg width="80" height="54" viewBox="0 0 80 54" fill="none">
                    <rect width="80" height="54" fill="transparent"/>
                    <path d="M15 52 L15 30 L4 30 L20 10 L36 30 L25 30 L25 52Z" fill="rgba(20,60,30,0.85)"/>
                    <path d="M42 52 L42 27 L30 27 L46 6 L62 27 L51 27 L51 52Z" fill="rgba(15,50,25,0.75)"/>
                    <path d="M65 52 L65 33 L56 33 L68 15 L80 33 L73 33 L73 52Z" fill="rgba(20,55,28,0.8)"/>
                    <rect x="0" y="50" width="80" height="4" fill="rgba(10,30,15,0.6)"/>
                    <line x1="0" y1="50" x2="80" y2="50" stroke="rgba(30,70,35,0.4)" stroke-width="0.5"/>
                  </svg>
                </div>
              {:else if project.thumb === 'city'}
                <div class="thumb-bg thumb-city">
                  <svg width="80" height="54" viewBox="0 0 80 54" fill="none">
                    <defs>
                      <linearGradient id="sky-c" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stop-color="#1a0a00"/>
                        <stop offset="60%" stop-color="#5a2500"/>
                        <stop offset="100%" stop-color="#a04010"/>
                      </linearGradient>
                    </defs>
                    <rect width="80" height="54" fill="url(#sky-c)"/>
                    <rect x="6"  y="25" width="8"  height="29" fill="rgba(8,8,18,0.95)"/>
                    <rect x="17" y="18" width="6"  height="36" fill="rgba(12,10,22,0.95)"/>
                    <rect x="27" y="28" width="10" height="26" fill="rgba(8,8,18,0.9)"/>
                    <rect x="42" y="14" width="7"  height="40" fill="rgba(10,8,20,0.95)"/>
                    <rect x="53" y="22" width="11" height="32" fill="rgba(8,8,18,0.9)"/>
                    <rect x="67" y="30" width="8"  height="24" fill="rgba(12,10,22,0.85)"/>
                  </svg>
                </div>
              {:else if project.thumb === 'eye'}
                <div class="thumb-bg thumb-eye">
                  <div style="
                    width:100%; height:100%;
                    background: radial-gradient(ellipse 55% 70% at 50% 50%, #8b4513 0%, #5c2800 30%, #1a0800 60%, #000 100%);
                    position:relative; overflow:hidden;
                  ">
                    <div style="
                      position:absolute; top:50%; left:50%;
                      transform: translate(-50%,-50%);
                      width:50px; height:50px;
                      border-radius:50%;
                      background: conic-gradient(from 0deg, #c47a2a, #8b4513, #d4882e, #6b3410, #c47a2a);
                    "></div>
                    <div style="
                      position:absolute; top:50%; left:50%;
                      transform: translate(-50%,-50%);
                      width:18px; height:18px;
                      border-radius:50%; background:#000;
                    "></div>
                    <div style="
                      position:absolute; top:calc(50% - 9px); left:calc(50% + 5px);
                      width:7px; height:7px; border-radius:50%;
                      background:rgba(255,255,255,0.7); filter:blur(1.5px);
                    "></div>
                  </div>
                </div>
              {:else if project.thumb === 'dark'}
                <div class="thumb-bg thumb-dark">
                  <div style="
                    width:100%; height:100%;
                    background: linear-gradient(135deg, #0a0f14 0%, #0d1c28 40%, #1a2840 70%, #0a1520 100%);
                    position:relative; overflow:hidden;
                  ">
                    <div style="
                      position:absolute; top:0; right:30%; bottom:0; width:1px;
                      background: linear-gradient(to bottom, transparent, rgba(100,160,255,0.3), transparent);
                    "></div>
                  </div>
                </div>
              {:else if project.thumb === 'desk'}
                <div class="thumb-bg thumb-desk">
                  <div style="
                    width:100%; height:100%;
                    background: linear-gradient(135deg, #080810, #10102a);
                    position:relative; overflow:hidden; display:flex; align-items:center; justify-content:center;
                  ">
                    <svg width="60" height="40" viewBox="0 0 60 40" fill="none">
                      <ellipse cx="30" cy="22" rx="8" ry="9" fill="rgba(25,25,45,0.9)"/>
                      <ellipse cx="30" cy="12" rx="5" ry="5" fill="rgba(30,30,50,0.85)"/>
                      <rect x="10" y="34" width="40" height="2" fill="rgba(35,35,60,0.7)" rx="1"/>
                      <ellipse cx="30" cy="32" rx="14" ry="4" fill="rgba(45,45,90,0.25)"/>
                    </svg>
                  </div>
                </div>
              {:else if project.thumb === 'mountain'}
                <div class="thumb-bg thumb-mountain">
                  <div style="
                    width:100%; height:100%;
                    background: linear-gradient(to bottom, #87ceeb 0%, #b8dff5 35%, #5a8a5a 60%, #3a6040 100%);
                    position:relative; overflow:hidden;
                  ">
                    <svg width="80" height="54" viewBox="0 0 80 54" fill="none" style="position:absolute;bottom:0;left:0;">
                      <path d="M0 36 L20 14 L40 36Z" fill="rgba(120,160,120,0.7)"/>
                      <path d="M25 36 L50 8 L75 36Z" fill="rgba(100,140,100,0.8)"/>
                      <path d="M55 36 L70 20 L80 36Z" fill="rgba(130,165,130,0.65)"/>
                      <rect x="0" y="36" width="80" height="18" fill="rgba(50,90,55,0.8)"/>
                    </svg>
                  </div>
                </div>
              {/if}
              <span class="duration-badge">{project.duration}</span>
            </div>

            <!-- Info -->
            <div class="project-info">
              <div class="project-title">{project.title}</div>
              <div class="project-path">{project.path}</div>
              <div class="project-meta">
                Updated {project.updated}
                <span class="meta-dot">•</span>
                {project.resolution}
                <span class="meta-dot">•</span>
                {project.fps}
              </div>
              <div class="project-tags">
                {#each project.tags as tag}
                  {@const c = tagColorMap[project.tagColors[tag]] ?? tagColorMap.blue}
                  <span class="tag" style="background:{c.bg}; color:{c.color};">{tag}</span>
                {/each}
              </div>
            </div>

            <!-- Actions -->
            <button class="row-menu-btn" title="More options">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="12" cy="19" r="1"/>
              </svg>
            </button>

          </div>
        {/each}
      </div>

    {:else}
      <!-- Grid view -->
      <div class="projects-grid">
        {#each filtered as project (project.id)}
          <div class="project-card">
            <div class="card-thumb">
              {#if project.thumb === 'forest'}
                <div class="thumb-bg thumb-forest" style="height:100%;"></div>
              {:else if project.thumb === 'city'}
                <div class="thumb-bg thumb-city" style="height:100%;"></div>
              {:else if project.thumb === 'eye'}
                <div class="thumb-bg thumb-eye" style="height:100%; background:radial-gradient(ellipse 55% 70% at 50% 50%, #8b4513 0%, #5c2800 30%, #000 100%);"></div>
              {:else if project.thumb === 'dark'}
                <div class="thumb-bg thumb-dark" style="height:100%; background:linear-gradient(135deg,#0a0f14,#1a2840);"></div>
              {:else if project.thumb === 'desk'}
                <div class="thumb-bg thumb-desk" style="height:100%; background:linear-gradient(135deg,#080810,#10102a);"></div>
              {:else if project.thumb === 'mountain'}
                <div class="thumb-bg thumb-mountain" style="height:100%; background:linear-gradient(to bottom,#87ceeb,#5a8a5a,#3a6040);"></div>
              {/if}
              <span class="duration-badge">{project.duration}</span>
            </div>
            <div class="card-info">
              <div class="project-title">{project.title}</div>
              <div class="project-meta" style="margin-top:4px;">Updated {project.updated}</div>
              <div class="project-tags" style="margin-top:6px;">
                {#each project.tags as tag}
                  {@const c = tagColorMap[project.tagColors[tag]] ?? tagColorMap.blue}
                  <span class="tag" style="background:{c.bg}; color:{c.color};">{tag}</span>
                {/each}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <!-- Footer count -->
    <div class="list-footer">
      Showing 1 to {filtered.length} of {filtered.length} project{filtered.length !== 1 ? 's' : ''}
    </div>
  </div>

</div>

<!-- ── SRT Upload Modal ────────────────────────────────────── -->
{#if showModal}
  <!-- Backdrop -->
  <div class="modal-backdrop" on:click={closeModal} role="presentation"></div>

  <!-- Modal panel -->
  <div class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-title">

    <!-- Modal header -->
    <div class="modal-header">
      <div class="modal-header-left">
        <div class="modal-icon">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
          </svg>
        </div>
        <div>
          <h2 class="modal-title" id="modal-title">New Project</h2>
          <p class="modal-subtitle">Import a subtitle file to get started</p>
        </div>
      </div>
      <button class="modal-close" on:click={closeModal} aria-label="Close">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>

    <!-- Drop zone -->
    <div class="modal-body">
      <div
        class="drop-zone {isDragOver ? 'drag-over' : ''} {uploadedFile ? 'has-file' : ''} {fileError ? 'has-error' : ''}"
        on:drop={handleDrop}
        on:dragover={handleDragOver}
        on:dragleave={handleDragLeave}
        role="region"
        aria-label="File drop zone"
      >
        {#if uploadedFile}
          <!-- File accepted state -->
          <div class="dz-success">
            <div class="dz-file-icon">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14 2 14 8 20 8"/>
                <line x1="16" y1="13" x2="8" y2="13"/>
                <line x1="16" y1="17" x2="8" y2="17"/>
                <polyline points="10 9 9 9 8 9"/>
              </svg>
            </div>
            <div class="dz-file-info">
              <span class="dz-file-name">{uploadedFile.name}</span>
              <span class="dz-file-size">{formatBytes(uploadedFile.size)}</span>
            </div>
            <div class="dz-check">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"/>
              </svg>
            </div>
            <button class="dz-remove" on:click|stopPropagation={() => { uploadedFile = null; fileError = ''; }} aria-label="Remove file">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
        {:else}
          <!-- Default / error state -->
          <div class="dz-idle">
            <div class="dz-icon {fileError ? 'dz-icon-error' : ''}">
              {#if fileError}
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="10"/>
                  <line x1="12" y1="8" x2="12" y2="12"/>
                  <line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>
              {:else}
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="16 16 12 12 8 16"/>
                  <line x1="12" y1="12" x2="12" y2="21"/>
                  <path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3"/>
                </svg>
              {/if}
            </div>

            {#if fileError}
              <p class="dz-error-text">{fileError}</p>
              <p class="dz-hint">Please drop a valid <code>.srt</code> file</p>
            {:else if isDragOver}
              <p class="dz-main-text drag-active-text">Release to upload</p>
            {:else}
              <p class="dz-main-text">Drag & drop your <code>.srt</code> file here</p>
              <p class="dz-hint">or click below to browse</p>
            {/if}

            <label class="browse-btn">
              <input
                type="file"
                accept=".srt"
                style="display:none"
                on:change={handleFileInput}
              />
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="17 8 12 3 7 8"/>
                <line x1="12" y1="3" x2="12" y2="15"/>
              </svg>
              Browse for file
            </label>
          </div>
        {/if}
      </div>

      <!-- Format note -->
      <div class="format-note">
        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/>
        </svg>
        Only <strong>.srt</strong> subtitle files are supported
      </div>
    </div>

    <!-- Modal footer -->
    <div class="modal-footer">
      <button class="btn-cancel" on:click={closeModal}>Cancel</button>
      <button
        class="btn-create"
        disabled={!uploadedFile}
        on:click={handleCreate}
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
        Create Project
      </button>
    </div>

  </div>
{/if}

<style>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;600&family=Syne:wght@400;500;600;700&display=swap');

/* Page wrapper */
.projects-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  background: var(--bg-void, #f0f1f3);
  font-family: 'Syne', sans-serif;
}

/* ── Header ─────────────────────────────────────────────── */
.projects-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 28px 16px;
  flex-shrink: 0;
  border-bottom: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  background: var(--bg-surface, #fff);
}
.page-title {
  font-size: 22px;
  font-weight: 700;
  color: var(--text-primary, #1a1d23);
  letter-spacing: -0.02em;
  line-height: 1.2;
}
.page-subtitle {
  font-size: 12.5px;
  color: var(--text-muted, #8a96a8);
  margin-top: 3px;
}
.create-btn {
  display: flex;
  align-items: center;
  gap: 7px;
  background: var(--accent-orange, #e05c10);
  color: #fff;
  font-family: 'Syne', sans-serif;
  font-size: 13px;
  font-weight: 600;
  padding: 9px 18px;
  border-radius: 5px;
  cursor: pointer;
  border: none;
  transition: background 120ms ease, transform 60ms;
  white-space: nowrap;
}
.create-btn:hover { background: var(--accent-orange2, #c44d08); }
.create-btn:active { transform: scale(0.97); }

/* ── Toolbar ─────────────────────────────────────────────── */
.projects-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 28px;
  flex-shrink: 0;
  background: var(--bg-surface, #fff);
  border-bottom: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
}
.section-label {
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--text-primary, #1a1d23);
}
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.toolbar-search {
  display: flex;
  align-items: center;
  gap: 6px;
  background: var(--bg-panel, #f4f5f7);
  border: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  border-radius: 5px;
  padding: 5px 10px;
  width: 170px;
}
.toolbar-search svg { color: var(--text-muted, #8a96a8); flex-shrink: 0; }
.toolbar-search input {
  background: none; border: none; outline: none;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11.5px;
  color: var(--text-secondary, #4a5568);
  width: 100%;
}
.toolbar-search input::placeholder { color: var(--text-muted, #8a96a8); }

.sort-wrap { display: flex; align-items: center; gap: 6px; }
.sort-label { font-size: 12px; color: var(--text-muted, #8a96a8); white-space: nowrap; }
.sort-select {
  font-family: 'Syne', sans-serif;
  font-size: 12.5px;
  font-weight: 500;
  color: var(--text-secondary, #4a5568);
  background: var(--bg-panel, #f4f5f7);
  border: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  border-radius: 5px;
  padding: 4px 28px 4px 10px;
  outline: none;
  cursor: pointer;
  -webkit-appearance: none;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='10' viewBox='0 0 24 24' fill='none' stroke='%238a96a8' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 8px center;
}

.view-toggle {
  display: flex; align-items: center; gap: 2px;
  background: var(--bg-panel, #f4f5f7);
  border: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  border-radius: 5px;
  padding: 2px;
}
.view-btn {
  width: 26px; height: 26px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 3px;
  color: var(--text-muted, #8a96a8);
  cursor: pointer;
  transition: color 120ms ease, background 120ms ease;
  border: none; background: none;
}
.view-btn:hover { color: var(--text-secondary, #4a5568); }
.view-btn.active { color: var(--text-primary, #1a1d23); background: var(--bg-surface, #fff); box-shadow: 0 1px 3px rgba(0,0,0,0.08); }

/* ── List / Grid wrap ────────────────────────────────────── */
.projects-list-wrap {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-surface, #fff);
}
.projects-list-wrap::-webkit-scrollbar { width: 4px; }
.projects-list-wrap::-webkit-scrollbar-thumb { background: var(--border-mid, rgba(0,0,0,0.11)); border-radius: 2px; }

.projects-list { display: flex; flex-direction: column; }

.project-row {
  display: flex;
  align-items: center;
  gap: 18px;
  padding: 14px 28px;
  border-bottom: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  cursor: pointer;
  transition: background 120ms ease;
}
.project-row:hover { background: var(--bg-void, #f0f1f3); }

.project-thumb {
  width: 150px; height: 100px;
  border-radius: 5px;
  overflow: hidden;
  flex-shrink: 0;
  position: relative;
  background: #111;
}
.thumb-bg { width: 100%; height: 100%; }
.thumb-forest   { background: linear-gradient(135deg, #1a2e20 0%, #0f2010 60%, #050e07 100%); }
.thumb-city     { background: linear-gradient(to bottom, #1a0a00 0%, #4a1a00 60%, #8a3000 100%); }
.thumb-eye      { background: radial-gradient(ellipse 55% 70% at 50% 50%, #8b4513 0%, #5c2800 30%, #000 100%); }
.thumb-dark     { background: linear-gradient(135deg, #0a0f14 0%, #0d1c28 40%, #1a2840 70%, #0a1520 100%); }
.thumb-desk     { background: linear-gradient(135deg, #080810 0%, #10102a 100%); }
.thumb-mountain { background: linear-gradient(to bottom, #87ceeb 0%, #b8dff5 35%, #5a8a5a 60%, #3a6040 100%); }
.thumb-bg svg { width: 100%; height: 100%; }

.duration-badge {
  position: absolute;
  bottom: 6px; right: 6px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px; font-weight: 600;
  color: #fff;
  background: rgba(0,0,0,0.6);
  padding: 2px 6px;
  border-radius: 3px;
  letter-spacing: 0.04em;
}

.project-info { flex: 1; min-width: 0; }
.project-title {
  font-size: 14.5px; font-weight: 600;
  color: var(--text-primary, #1a1d23);
  margin-bottom: 2px; letter-spacing: -0.01em;
}
.project-path {
  font-family: 'JetBrains Mono', monospace;
  font-size: 10.5px; color: var(--text-muted, #8a96a8); margin-bottom: 4px;
}
.project-meta {
  font-size: 11.5px; color: var(--text-muted, #8a96a8);
  margin-bottom: 7px;
  display: flex; align-items: center; gap: 5px; flex-wrap: wrap;
}
.meta-dot { width: 3px; height: 3px; border-radius: 50%; background: var(--text-tiny, #b0bac8); }
.project-tags { display: flex; flex-wrap: wrap; gap: 5px; }
.tag {
  font-size: 10.5px; font-weight: 600;
  padding: 2px 9px; border-radius: 100px; letter-spacing: 0.03em;
}

.row-menu-btn {
  width: 30px; height: 30px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 5px; color: var(--text-muted, #8a96a8);
  opacity: 0;
  transition: opacity 120ms ease, color 120ms ease, background 120ms ease;
  cursor: pointer; flex-shrink: 0; border: none; background: none;
}
.project-row:hover .row-menu-btn { opacity: 1; }
.row-menu-btn:hover { color: var(--text-primary, #1a1d23); background: var(--bg-hover, #e2e5ea); }

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 16px; padding: 20px 28px;
}
.project-card {
  border-radius: 7px; overflow: hidden;
  border: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  background: var(--bg-surface, #fff);
  cursor: pointer;
  transition: box-shadow 120ms ease, transform 120ms ease;
  box-shadow: 0 1px 4px rgba(0,0,0,0.07);
}
.project-card:hover { box-shadow: 0 4px 16px rgba(0,0,0,0.12); transform: translateY(-1px); }
.card-thumb {
  width: 100%; padding-top: 56.25%;
  position: relative; overflow: hidden; background: #111;
}
.card-thumb .thumb-bg, .card-thumb > div { position: absolute; inset: 0; }
.card-info { padding: 10px 12px 12px; }

.list-footer {
  padding: 14px 28px;
  font-size: 11.5px; color: var(--text-muted, #8a96a8);
  text-align: center;
  border-top: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
}

/* ══════════════════════════════════════════════════════════
   MODAL
   ══════════════════════════════════════════════════════════ */

.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(10, 12, 18, 0.5);
  backdrop-filter: blur(3px);
  -webkit-backdrop-filter: blur(3px);
  z-index: 100;
  animation: backdrop-in 160ms ease;
}

@keyframes backdrop-in {
  from { opacity: 0; }
  to   { opacity: 1; }
}

.modal {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 101;
  width: 460px;
  max-width: calc(100vw - 32px);
  background: var(--bg-surface, #fff);
  border-radius: 10px;
  box-shadow:
    0 0 0 1px rgba(0,0,0,0.07),
    0 8px 32px rgba(0,0,0,0.15),
    0 2px 8px rgba(0,0,0,0.08);
  font-family: 'Syne', sans-serif;
  animation: modal-in 180ms cubic-bezier(0.34, 1.28, 0.64, 1);
  overflow: hidden;
}

@keyframes modal-in {
  from { opacity: 0; transform: translate(-50%, -46%) scale(0.96); }
  to   { opacity: 1; transform: translate(-50%, -50%) scale(1); }
}

/* Modal header */
.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 16px;
  border-bottom: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
}
.modal-header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}
.modal-icon {
  width: 32px; height: 32px;
  display: flex; align-items: center; justify-content: center;
  background: rgba(224, 92, 16, 0.10);
  color: #e05c10;
  border-radius: 7px;
  flex-shrink: 0;
}
.modal-title {
  font-size: 15px;
  font-weight: 700;
  color: var(--text-primary, #1a1d23);
  letter-spacing: -0.01em;
  line-height: 1.2;
}
.modal-subtitle {
  font-size: 11.5px;
  color: var(--text-muted, #8a96a8);
  margin-top: 1px;
}
.modal-close {
  width: 28px; height: 28px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 5px;
  color: var(--text-muted, #8a96a8);
  border: none; background: none; cursor: pointer;
  transition: color 120ms ease, background 120ms ease;
}
.modal-close:hover { color: var(--text-primary, #1a1d23); background: var(--bg-void, #f0f1f3); }

/* Modal body */
.modal-body {
  padding: 20px;
}

/* Drop zone */
.drop-zone {
  border: 1.5px dashed var(--border-mid, rgba(0,0,0,0.14));
  border-radius: 8px;
  background: var(--bg-panel, #f8f9fa);
  min-height: 160px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition:
    border-color 120ms ease,
    background 120ms ease,
    box-shadow 120ms ease;
  position: relative;
  overflow: hidden;
}
.drop-zone.drag-over {
  border-color: #e05c10;
  background: rgba(224, 92, 16, 0.04);
  box-shadow: 0 0 0 4px rgba(224, 92, 16, 0.08);
}
.drop-zone.has-file {
  border-style: solid;
  border-color: rgba(26, 153, 85, 0.35);
  background: rgba(26, 153, 85, 0.03);
}
.drop-zone.has-error {
  border-color: rgba(217, 48, 37, 0.35);
  background: rgba(217, 48, 37, 0.03);
}

/* Idle / error state */
.dz-idle {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 28px 20px;
  text-align: center;
  width: 100%;
}
.dz-icon {
  width: 56px; height: 56px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 50%;
  background: rgba(0,0,0,0.04);
  color: var(--text-muted, #8a96a8);
  margin-bottom: 4px;
}
.dz-icon-error {
  background: rgba(217, 48, 37, 0.08);
  color: #d93025;
}
.dz-main-text {
  font-size: 13.5px;
  font-weight: 600;
  color: var(--text-primary, #1a1d23);
  letter-spacing: -0.01em;
}
.dz-main-text code, .dz-hint code {
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.9em;
  background: rgba(0,0,0,0.06);
  padding: 1px 5px;
  border-radius: 3px;
}
.drag-active-text { color: #e05c10; }
.dz-hint {
  font-size: 11.5px;
  color: var(--text-muted, #8a96a8);
  margin-top: -2px;
}
.dz-error-text {
  font-size: 12.5px;
  font-weight: 600;
  color: #d93025;
}

.browse-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  margin-top: 6px;
  padding: 7px 16px;
  background: #fff;
  border: 1px solid var(--border-mid, rgba(0,0,0,0.12));
  border-radius: 5px;
  font-family: 'Syne', sans-serif;
  font-size: 12.5px;
  font-weight: 600;
  color: var(--text-secondary, #4a5568);
  cursor: pointer;
  transition: background 120ms ease, border-color 120ms ease, color 120ms ease, box-shadow 120ms ease;
  box-shadow: 0 1px 2px rgba(0,0,0,0.06);
}
.browse-btn:hover {
  background: var(--bg-void, #f0f1f3);
  border-color: rgba(0,0,0,0.18);
  color: var(--text-primary, #1a1d23);
}

/* Success state */
.dz-success {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 18px;
  width: 100%;
}
.dz-file-icon {
  width: 40px; height: 40px;
  display: flex; align-items: center; justify-content: center;
  background: rgba(26, 153, 85, 0.10);
  color: #1a9955;
  border-radius: 7px;
  flex-shrink: 0;
}
.dz-file-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.dz-file-name {
  font-family: 'JetBrains Mono', monospace;
  font-size: 12.5px;
  font-weight: 500;
  color: var(--text-primary, #1a1d23);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.dz-file-size {
  font-size: 11px;
  color: var(--text-muted, #8a96a8);
}
.dz-check {
  width: 26px; height: 26px;
  display: flex; align-items: center; justify-content: center;
  background: rgba(26, 153, 85, 0.12);
  color: #1a9955;
  border-radius: 50%;
  flex-shrink: 0;
}
.dz-remove {
  width: 26px; height: 26px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 5px;
  color: var(--text-muted, #8a96a8);
  border: none; background: none; cursor: pointer;
  transition: color 120ms ease, background 120ms ease;
  flex-shrink: 0;
}
.dz-remove:hover { color: #d93025; background: rgba(217, 48, 37, 0.07); }

/* Format note */
.format-note {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 10px;
  font-size: 11px;
  color: var(--text-muted, #8a96a8);
}
.format-note svg { flex-shrink: 0; }
.format-note strong { font-weight: 600; color: var(--text-secondary, #4a5568); }

/* Modal footer */
.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  padding: 14px 20px;
  border-top: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  background: var(--bg-panel, #f8f9fa);
}
.btn-cancel {
  padding: 8px 16px;
  font-family: 'Syne', sans-serif;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary, #4a5568);
  background: #fff;
  border: 1px solid var(--border-mid, rgba(0,0,0,0.12));
  border-radius: 5px;
  cursor: pointer;
  transition: background 120ms ease, color 120ms ease;
}
.btn-cancel:hover { background: var(--bg-void, #f0f1f3); color: var(--text-primary, #1a1d23); }

.btn-create {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 18px;
  font-family: 'Syne', sans-serif;
  font-size: 13px;
  font-weight: 600;
  color: #fff;
  background: #e05c10;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background 120ms ease, transform 60ms, opacity 120ms ease;
}
.btn-create:hover:not(:disabled) { background: #c44d08; }
.btn-create:active:not(:disabled) { transform: scale(0.97); }
.btn-create:disabled {
  opacity: 0.38;
  cursor: not-allowed;
}
</style>