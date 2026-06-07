<script>
import { goto } from '$app/navigation';
import { invoke } from '@tauri-apps/api/core';

// ── State ─────────────────────────────────────────────────
let viewMode = 'list';
let sortBy   = 'Last Modified';
let searchQuery = '';

// Modal state
let showModal       = false;
let modalStep       = 1;
let isDragOver      = false;
let isDragOverVideo = false;
let uploadedSrt     = null;
let uploadedVideo   = null;
let srtError        = '';
let videoError      = '';
let projectTitle    = 'Untitled';

// Saved destination paths returned by Rust (real OS paths)
let srtSavedPath   = '';   // e.g. C:\Users\...\projects\Untitled\srt\foo.srt
let videoSavedPath = '';

const sortOptions = ['Last Modified', 'Name', 'Duration', 'Created'];
const ACCEPTED_VIDEO_EXTS = ['.mp4', '.mkv', '.mov', '.avi', '.webm', '.m4v'];

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

// ── File → bytes helper ────────────────────────────────────
/**
 * Read a File object into a Uint8Array via FileReader.
 * This is the only reliable way to get file content in Tauri v2,
 * since the webview never exposes real OS paths.
 */
function readFileBytes(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload  = () => resolve(new Uint8Array(reader.result));
    reader.onerror = () => reject(reader.error);
    reader.readAsArrayBuffer(file);
  });
}

/**
 * Strip the filename from a full path to get the directory.
 * Works for both / and \ separators.
 */
function dirOf(fullPath) {
  const last = Math.max(fullPath.lastIndexOf('/'), fullPath.lastIndexOf('\\'));
  return last > 0 ? fullPath.slice(0, last) : fullPath;
}

// ── Modal helpers ──────────────────────────────────────────
function openModal() {
  showModal       = true;
  modalStep       = 1;
  uploadedSrt     = null;
  uploadedVideo   = null;
  srtError        = '';
  videoError      = '';
  isDragOver      = false;
  isDragOverVideo = false;
  projectTitle    = 'Untitled';
  srtSavedPath    = '';
  videoSavedPath  = '';
}

function closeModal() {
  showModal       = false;
  uploadedSrt     = null;
  uploadedVideo   = null;
  srtError        = '';
  videoError      = '';
  isDragOver      = false;
  isDragOverVideo = false;
  srtSavedPath    = '';
  videoSavedPath  = '';
}

// ── SRT helpers ────────────────────────────────────────────
function validateAndSetSrt(file) {
  srtError = '';
  if (!file) return;
  if (!file.name.toLowerCase().endsWith('.srt')) {
    srtError = 'Only .srt subtitle files are accepted.';
    uploadedSrt = null;
    return;
  }
  uploadedSrt = file;
}

function handleDrop(e)      { e.preventDefault(); isDragOver = false; validateAndSetSrt(e.dataTransfer.files[0]); }
function handleDragOver(e)  { e.preventDefault(); isDragOver = true; }
function handleDragLeave()  { isDragOver = false; }
function handleSrtInput(e)  { validateAndSetSrt(e.target.files[0]); }

// ── Video helpers ──────────────────────────────────────────
function validateAndSetVideo(file) {
  videoError = '';
  if (!file) return;
  const ext = '.' + file.name.split('.').pop().toLowerCase();
  if (!ACCEPTED_VIDEO_EXTS.includes(ext)) {
    videoError = `Unsupported format. Accepted: ${ACCEPTED_VIDEO_EXTS.join(', ')}`;
    uploadedVideo = null;
    return;
  }
  uploadedVideo = file;
}

function handleVideoDrop(e)      { e.preventDefault(); isDragOverVideo = false; validateAndSetVideo(e.dataTransfer.files[0]); }
function handleVideoDragOver(e)  { e.preventDefault(); isDragOverVideo = true; }
function handleVideoDragLeave()  { isDragOverVideo = false; }
function handleVideoInput(e)     { validateAndSetVideo(e.target.files[0]); }

// ── Step progression ───────────────────────────────────────
async function confirmSrt() {
  if (!uploadedSrt) return;

  try {
    const bytes = await readFileBytes(uploadedSrt);
    // invoke returns the saved destination path from Rust
    const savedPath = await invoke('import_file_bytes', {
      fileKind:    'srt',
      fileName:    uploadedSrt.name,
      bytes:       Array.from(bytes),   // Tauri serialises Vec<u8> from JS number[]
      projectName: projectTitle || 'Untitled',
    });
    srtSavedPath = savedPath;
    console.log('[snipSnap] SRT saved to:', savedPath);
  } catch (e) {
    console.error('[snipSnap] SRT import error:', e);
    srtError = 'Failed to save file: ' + e;
    return;
  }

  modalStep = 2;
}

async function confirmVideo() {
 /*
  if (!uploadedVideo) return;

  try {
    const bytes = await readFileBytes(uploadedVideo);
    const savedPath = await invoke('import_file_bytes', {
      fileKind:    'video',
      fileName:    uploadedVideo.name,
      bytes:       Array.from(bytes),
      projectName: projectTitle || 'Untitled',
    });
    videoSavedPath = savedPath;
    console.log('[snipSnap] Video saved to:', savedPath);
  } catch (e) {
    console.error('[snipSnap] Video import error:', e);
    videoError = 'Failed to save file: ' + e;
    return;
  }
    */

  modalStep = 3;
}

function skipVideo() {
  modalStep = 3;
}

function proceedToTimeline() {
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
      <div class="toolbar-search">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input type="text" placeholder="Filter projects..." bind:value={searchQuery} />
      </div>
      <div class="sort-wrap">
        <span class="sort-label">Sort by:</span>
        <select class="sort-select" bind:value={sortBy}>
          {#each sortOptions as opt}
            <option value={opt}>{opt}</option>
          {/each}
        </select>
      </div>
      <div class="view-toggle">
        <button class="view-btn {viewMode === 'grid' ? 'active' : ''}" title="Grid view" on:click={() => viewMode = 'grid'}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/>
            <rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/>
          </svg>
        </button>
        <button class="view-btn {viewMode === 'list' ? 'active' : ''}" title="List view" on:click={() => viewMode = 'list'}>
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
            <div class="project-thumb">
              {#if project.thumb === 'forest'}
                <div class="thumb-bg thumb-forest">
                  <svg width="80" height="54" viewBox="0 0 80 54" fill="none">
                    <rect width="80" height="54" fill="transparent"/>
                    <path d="M15 52 L15 30 L4 30 L20 10 L36 30 L25 30 L25 52Z" fill="rgba(20,60,30,0.85)"/>
                    <path d="M42 52 L42 27 L30 27 L46 6 L62 27 L51 27 L51 52Z" fill="rgba(15,50,25,0.75)"/>
                    <path d="M65 52 L65 33 L56 33 L68 15 L80 33 L73 33 L73 52Z" fill="rgba(20,55,28,0.8)"/>
                    <rect x="0" y="50" width="80" height="4" fill="rgba(10,30,15,0.6)"/>
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
                  <div style="width:100%;height:100%;background:radial-gradient(ellipse 55% 70% at 50% 50%,#8b4513 0%,#5c2800 30%,#1a0800 60%,#000 100%);position:relative;overflow:hidden;">
                    <div style="position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);width:50px;height:50px;border-radius:50%;background:conic-gradient(from 0deg,#c47a2a,#8b4513,#d4882e,#6b3410,#c47a2a);"></div>
                    <div style="position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);width:18px;height:18px;border-radius:50%;background:#000;"></div>
                  </div>
                </div>
              {:else if project.thumb === 'dark'}
                <div class="thumb-bg thumb-dark"></div>
              {:else if project.thumb === 'desk'}
                <div class="thumb-bg thumb-desk"></div>
              {:else if project.thumb === 'mountain'}
                <div class="thumb-bg thumb-mountain"></div>
              {/if}
              <span class="duration-badge">{project.duration}</span>
            </div>
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
            <button class="row-menu-btn" title="More options">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="12" cy="19" r="1"/>
              </svg>
            </button>
          </div>
        {/each}
      </div>
    {:else}
      <div class="projects-grid">
        {#each filtered as project (project.id)}
          <div class="project-card">
            <div class="card-thumb">
              <div class="thumb-bg thumb-{project.thumb}" style="position:absolute;inset:0;height:100%;"></div>
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
    <div class="list-footer">
      Showing 1 to {filtered.length} of {filtered.length} project{filtered.length !== 1 ? 's' : ''}
    </div>
  </div>

</div>

<!-- ══════════════════════════════════════════════════════════
     MODAL
     ══════════════════════════════════════════════════════════ -->
{#if showModal}
  <div class="modal-backdrop" on:click={closeModal} role="presentation"></div>

  <div class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-title">

    <div class="modal-header">
      <div class="modal-header-left">
        <div class="modal-icon">
          {#if modalStep === 1}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
          {:else if modalStep === 2}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polygon points="23 7 16 12 23 17 23 7"/><rect x="1" y="5" width="15" height="14" rx="2" ry="2"/>
            </svg>
          {:else}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 11 12 14 22 4"/>
              <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/>
            </svg>
          {/if}
        </div>
        <div>
          <h2 class="modal-title" id="modal-title">
            {#if modalStep === 1}New Project
            {:else if modalStep === 2}Add Episode Video
            {:else}Project Summary
            {/if}
          </h2>
          <p class="modal-subtitle">
            {#if modalStep === 1}
              Step 1 of 2 — Import a subtitle file to get started
            {:else if modalStep === 2}
              Step 2 of 2 — Drop your source episode or movie file
            {:else}
              Files saved — ready to open the timeline
            {/if}
          </p>
        </div>
      </div>
      <div class="modal-header-right">
        <div class="step-pills">
          <div class="step-pill {modalStep >= 1 ? 'active' : ''}">1</div>
          <div class="step-connector {modalStep >= 2 ? 'done' : ''}"></div>
          <div class="step-pill {modalStep >= 2 ? 'active' : ''}">2</div>
        </div>
        <button class="modal-close" on:click={closeModal} aria-label="Close">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- ── Step 1: SRT ── -->
    {#if modalStep === 1}
      <div class="modal-body">
        <div class="field-group">
          <label class="field-label" for="proj-name">Project Name</label>
          <input id="proj-name" class="field-input" type="text" placeholder="Untitled" bind:value={projectTitle} />
        </div>

        <div
          class="drop-zone {isDragOver ? 'drag-over' : ''} {uploadedSrt ? 'has-file' : ''} {srtError ? 'has-error' : ''}"
          on:drop={handleDrop}
          on:dragover={handleDragOver}
          on:dragleave={handleDragLeave}
          role="region"
          aria-label="SRT file drop zone"
        >
          {#if uploadedSrt}
            <div class="dz-success">
              <div class="dz-file-icon">
                <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                  <polyline points="14 2 14 8 20 8"/>
                  <line x1="16" y1="13" x2="8" y2="13"/>
                  <line x1="16" y1="17" x2="8" y2="17"/>
                </svg>
              </div>
              <div class="dz-file-info">
                <span class="dz-file-name">{uploadedSrt.name}</span>
                <span class="dz-file-size">{formatBytes(uploadedSrt.size)}</span>
              </div>
              <div class="dz-check">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
              </div>
              <button class="dz-remove" on:click|stopPropagation={() => { uploadedSrt = null; srtError = ''; }} aria-label="Remove file">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            </div>
          {:else}
            <div class="dz-idle">
              <div class="dz-icon {srtError ? 'dz-icon-error' : ''}">
                {#if srtError}
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
              {#if srtError}
                <p class="dz-error-text">{srtError}</p>
                <p class="dz-hint">Please drop a valid <code>.srt</code> file</p>
              {:else if isDragOver}
                <p class="dz-main-text drag-active-text">Release to upload</p>
              {:else}
                <p class="dz-main-text">Drag & drop your <code>.srt</code> file here</p>
                <p class="dz-hint">or click below to browse</p>
              {/if}
              <label class="browse-btn">
                <input type="file" accept=".srt" style="display:none" on:change={handleSrtInput} />
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

        <div class="format-note">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          Saved to <strong>~/Documents/donalds/projects/{projectTitle || 'Untitled'}/srt/</strong>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" on:click={closeModal}>Cancel</button>
        <button class="btn-create" disabled={!uploadedSrt} on:click={confirmSrt}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="9 18 15 12 9 6"/>
          </svg>
          Continue
        </button>
      </div>
    {/if}

    <!-- ── Step 2: Video ── -->
    {#if modalStep === 2}
      <div class="modal-body">
        <div
          class="drop-zone {isDragOverVideo ? 'drag-over' : ''} {uploadedVideo ? 'has-file' : ''} {videoError ? 'has-error' : ''}"
          on:drop={handleVideoDrop}
          on:dragover={handleVideoDragOver}
          on:dragleave={handleVideoDragLeave}
          role="region"
          aria-label="Video file drop zone"
        >
          {#if uploadedVideo}
            <div class="dz-success">
              <div class="dz-file-icon video-icon">
                <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <polygon points="23 7 16 12 23 17 23 7"/><rect x="1" y="5" width="15" height="14" rx="2" ry="2"/>
                </svg>
              </div>
              <div class="dz-file-info">
                <span class="dz-file-name">{uploadedVideo.name}</span>
                <span class="dz-file-size">{formatBytes(uploadedVideo.size)}</span>
              </div>
              <div class="dz-check">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
              </div>
              <button class="dz-remove" on:click|stopPropagation={() => { uploadedVideo = null; videoError = ''; }} aria-label="Remove file">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            </div>
          {:else}
            <div class="dz-idle">
              <div class="dz-icon {videoError ? 'dz-icon-error' : ''}">
                {#if videoError}
                  <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="10"/>
                    <line x1="12" y1="8" x2="12" y2="12"/>
                    <line x1="12" y1="16" x2="12.01" y2="16"/>
                  </svg>
                {:else}
                  <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <polygon points="23 7 16 12 23 17 23 7"/><rect x="1" y="5" width="15" height="14" rx="2" ry="2"/>
                  </svg>
                {/if}
              </div>
              {#if videoError}
                <p class="dz-error-text">{videoError}</p>
                <p class="dz-hint">Accepted: mp4, mkv, mov, avi, webm, m4v</p>
              {:else if isDragOverVideo}
                <p class="dz-main-text drag-active-text">Release to upload</p>
              {:else}
                <p class="dz-main-text">Drop your episode or movie file here</p>
                <p class="dz-hint">mp4 · mkv · mov · avi · webm · m4v</p>
              {/if}
              <label class="browse-btn">
                <input type="file" accept=".mp4,.mkv,.mov,.avi,.webm,.m4v" style="display:none" on:change={handleVideoInput} />
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

        <div class="format-note">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          Saved to <strong>~/Documents/donalds/projects/{projectTitle || 'Untitled'}/video/</strong>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" on:click={skipVideo}>Skip for now</button>
        <button class="btn-create" disabled={!uploadedVideo} on:click={confirmVideo}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
          </svg>
          Create Project
        </button>
      </div>
    {/if}

    <!-- ── Step 3: Summary ── -->
    {#if modalStep === 3}
      <div class="modal-body">
        <div class="review-project-name">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
          <span>{projectTitle || 'Untitled'}</span>
        </div>

        <div class="review-rows">

          <!-- SRT row — shows the real saved path from Rust -->
          <div class="review-row">
            <div class="review-row-icon srt-icon">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14 2 14 8 20 8"/>
                <line x1="16" y1="13" x2="8" y2="13"/>
                <line x1="16" y1="17" x2="8" y2="17"/>
              </svg>
            </div>
            <div class="review-row-body">
              <div class="review-row-label">Subtitle File</div>
              <div class="review-row-filename">{uploadedSrt?.name ?? '—'}</div>
              {#if srtSavedPath}
                <div class="review-row-dir">
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                  </svg>
                  <span class="dir-path">{dirOf(srtSavedPath)}</span>
                </div>
              {/if}
            </div>
            <div class="review-row-badge review-badge-srt">SRT</div>
          </div>

          <!-- Video row — shows the real saved path from Rust -->
          <div class="review-row {!uploadedVideo ? 'review-row-skipped' : ''}">
            <div class="review-row-icon video-icon-rev">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polygon points="23 7 16 12 23 17 23 7"/><rect x="1" y="5" width="15" height="14" rx="2" ry="2"/>
              </svg>
            </div>
            <div class="review-row-body">
              <div class="review-row-label">Video File</div>
              <div class="review-row-filename">{uploadedVideo?.name ?? 'Not provided — can be added later'}</div>
              {#if videoSavedPath}
                <div class="review-row-dir">
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                  </svg>
                  <span class="dir-path">{dirOf(videoSavedPath)}</span>
                </div>
              {/if}
            </div>
            {#if uploadedVideo}
              <div class="review-row-badge review-badge-video">VIDEO</div>
            {:else}
              <div class="review-row-badge review-badge-skip">SKIP</div>
            {/if}
          </div>

        </div>

        <div class="review-dest-note">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          Project folder: <strong>~/Documents/donalds/projects/{projectTitle || 'Untitled'}/</strong>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" on:click={closeModal}>Cancel</button>
        <button class="btn-create" on:click={proceedToTimeline}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="9 18 15 12 9 6"/>
          </svg>
          Open Timeline
        </button>
      </div>
    {/if}

  </div>
{/if}

<style>
/* ── Review step styles ──────────────────────────────────── */
.review-project-name {
  display: flex;
  align-items: center;
  gap: 7px;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #e8eaed);
  background: var(--surface-2, rgba(255,255,255,0.05));
  border: 1px solid var(--border, rgba(255,255,255,0.08));
  border-radius: 8px;
  padding: 10px 14px;
  margin-bottom: 14px;
}
.review-project-name svg { opacity: 0.5; flex-shrink: 0; }

.review-rows {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 14px;
}

.review-row {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  background: var(--surface-2, rgba(255,255,255,0.04));
  border: 1px solid var(--border, rgba(255,255,255,0.08));
  border-radius: 10px;
  padding: 12px 14px;
  transition: opacity 0.15s;
}
.review-row-skipped { opacity: 0.55; }

.review-row-icon {
  width: 34px;
  height: 34px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  margin-top: 1px;
}
.srt-icon      { background: rgba(26,114,232,0.12); color: #5a9ff5; border: 1px solid rgba(26,114,232,0.18); }
.video-icon-rev { background: rgba(124,82,200,0.12); color: #a57fde; border: 1px solid rgba(124,82,200,0.18); }

.review-row-body { flex: 1; min-width: 0; }
.review-row-label {
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted, rgba(232,234,237,0.4));
  margin-bottom: 3px;
}
.review-row-filename {
  font-size: 12.5px;
  font-weight: 500;
  color: var(--text-primary, #e8eaed);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 5px;
}
.review-row-dir {
  display: flex;
  align-items: center;
  gap: 5px;
  color: var(--text-muted, rgba(232,234,237,0.4));
}
.review-row-dir svg { flex-shrink: 0; opacity: 0.6; }
.dir-path {
  font-size: 11px;
  font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text-muted, rgba(232,234,237,0.45));
}

.review-row-badge {
  font-size: 9.5px;
  font-weight: 700;
  letter-spacing: 0.08em;
  border-radius: 5px;
  padding: 3px 7px;
  flex-shrink: 0;
  align-self: flex-start;
  margin-top: 2px;
}
.review-badge-srt   { background: rgba(26,114,232,0.12); color: #5a9ff5; border: 1px solid rgba(26,114,232,0.2); }
.review-badge-video { background: rgba(124,82,200,0.12); color: #a57fde; border: 1px solid rgba(124,82,200,0.2); }
.review-badge-skip  { background: rgba(255,255,255,0.05); color: rgba(232,234,237,0.35); border: 1px solid rgba(255,255,255,0.08); }

.review-dest-note {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--text-muted, rgba(232,234,237,0.4));
  padding: 9px 12px;
  background: var(--surface-2, rgba(255,255,255,0.03));
  border: 1px solid var(--border, rgba(255,255,255,0.06));
  border-radius: 7px;
}
.review-dest-note svg { opacity: 0.5; flex-shrink: 0; }
.review-dest-note strong { color: var(--text-secondary, rgba(232,234,237,0.65)); font-weight: 500; }



@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;600&family=Syne:wght@400;500;600;700&display=swap');

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

/* ── Project list / grid ─────────────────────────────────── */
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
  position: fixed; inset: 0;
  background: rgba(10,12,18,0.5);
  backdrop-filter: blur(3px);
  -webkit-backdrop-filter: blur(3px);
  z-index: 100;
  animation: backdrop-in 160ms ease;
}
@keyframes backdrop-in { from { opacity: 0; } to { opacity: 1; } }

.modal {
  position: fixed;
  top: 50%; left: 50%;
  transform: translate(-50%, -50%);
  z-index: 101;
  width: 480px;
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

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 16px;
  border-bottom: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
}
.modal-header-left { display: flex; align-items: center; gap: 12px; }
.modal-header-right { display: flex; align-items: center; gap: 12px; }
.modal-icon {
  width: 32px; height: 32px;
  display: flex; align-items: center; justify-content: center;
  background: rgba(224,92,16,0.10);
  color: #e05c10;
  border-radius: 7px;
  flex-shrink: 0;
}
.modal-title {
  font-size: 15px; font-weight: 700;
  color: var(--text-primary, #1a1d23);
  letter-spacing: -0.01em; line-height: 1.2;
}
.modal-subtitle { font-size: 11.5px; color: var(--text-muted, #8a96a8); margin-top: 1px; }
.modal-close {
  width: 28px; height: 28px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 5px;
  color: var(--text-muted, #8a96a8);
  border: none; background: none; cursor: pointer;
  transition: color 120ms ease, background 120ms ease;
}
.modal-close:hover { color: var(--text-primary, #1a1d23); background: var(--bg-void, #f0f1f3); }

/* Step pills */
.step-pills {
  display: flex; align-items: center; gap: 4px;
}
.step-pill {
  width: 22px; height: 22px;
  border-radius: 50%;
  display: flex; align-items: center; justify-content: center;
  font-size: 11px; font-weight: 700;
  background: var(--bg-panel, #f4f5f7);
  color: var(--text-muted, #8a96a8);
  border: 1.5px solid var(--border-subtle, rgba(0,0,0,0.1));
  transition: all 200ms ease;
}
.step-pill.active {
  background: #e05c10;
  color: #fff;
  border-color: #e05c10;
}
.step-connector {
  width: 18px; height: 2px;
  background: var(--border-subtle, rgba(0,0,0,0.1));
  border-radius: 1px;
  transition: background 200ms ease;
}
.step-connector.done { background: #e05c10; }

/* Modal body */
.modal-body { padding: 20px; }

/* Project name field */
.field-group { margin-bottom: 14px; }
.field-label {
  display: block;
  font-size: 11.5px; font-weight: 600;
  color: var(--text-secondary, #4a5568);
  margin-bottom: 5px; letter-spacing: 0.03em;
  text-transform: uppercase;
}
.field-input {
  width: 100%;
  font-family: 'Syne', sans-serif;
  font-size: 13.5px; font-weight: 500;
  color: var(--text-primary, #1a1d23);
  background: var(--bg-panel, #f8f9fa);
  border: 1px solid var(--border-subtle, rgba(0,0,0,0.1));
  border-radius: 5px;
  padding: 9px 12px;
  outline: none;
  box-sizing: border-box;
  transition: border-color 120ms ease, box-shadow 120ms ease;
}
.field-input:focus {
  border-color: #e05c10;
  box-shadow: 0 0 0 3px rgba(224,92,16,0.10);
}
.field-input::placeholder { color: var(--text-muted, #8a96a8); }

/* Drop zone */
.drop-zone {
  border: 1.5px dashed var(--border-mid, rgba(0,0,0,0.14));
  border-radius: 8px;
  background: var(--bg-panel, #f8f9fa);
  min-height: 148px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: border-color 120ms ease, background 120ms ease, box-shadow 120ms ease;
  position: relative;
  overflow: hidden;
}
.drop-zone.drag-over {
  border-color: #e05c10;
  background: rgba(224,92,16,0.04);
  box-shadow: 0 0 0 4px rgba(224,92,16,0.08);
}
.drop-zone.has-file {
  border-style: solid;
  border-color: rgba(26,153,85,0.35);
  background: rgba(26,153,85,0.03);
}
.drop-zone.has-error {
  border-color: rgba(217,48,37,0.35);
  background: rgba(217,48,37,0.03);
}

.dz-idle {
  display: flex; flex-direction: column; align-items: center;
  gap: 8px; padding: 24px 20px; text-align: center; width: 100%;
}
.dz-icon {
  width: 52px; height: 52px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 50%;
  background: rgba(0,0,0,0.04);
  color: var(--text-muted, #8a96a8);
  margin-bottom: 2px;
}
.dz-icon-error { background: rgba(217,48,37,0.08); color: #d93025; }
.dz-main-text {
  font-size: 13.5px; font-weight: 600;
  color: var(--text-primary, #1a1d23); letter-spacing: -0.01em;
}
.dz-main-text code, .dz-hint code {
  font-family: 'JetBrains Mono', monospace; font-size: 0.9em;
  background: rgba(0,0,0,0.06); padding: 1px 5px; border-radius: 3px;
}
.drag-active-text { color: #e05c10; }
.dz-hint { font-size: 11.5px; color: var(--text-muted, #8a96a8); margin-top: -2px; }
.dz-error-text { font-size: 12.5px; font-weight: 600; color: #d93025; }

.browse-btn {
  display: inline-flex; align-items: center; gap: 6px;
  margin-top: 6px; padding: 7px 16px;
  background: #fff;
  border: 1px solid var(--border-mid, rgba(0,0,0,0.12));
  border-radius: 5px;
  font-family: 'Syne', sans-serif; font-size: 12.5px; font-weight: 600;
  color: var(--text-secondary, #4a5568);
  cursor: pointer;
  transition: background 120ms ease, border-color 120ms ease, color 120ms ease;
  box-shadow: 0 1px 2px rgba(0,0,0,0.06);
}
.browse-btn:hover {
  background: var(--bg-void, #f0f1f3);
  border-color: rgba(0,0,0,0.18);
  color: var(--text-primary, #1a1d23);
}

.dz-success {
  display: flex; align-items: center; gap: 12px;
  padding: 16px 18px; width: 100%;
}
.dz-file-icon {
  width: 40px; height: 40px;
  display: flex; align-items: center; justify-content: center;
  background: rgba(26,153,85,0.10); color: #1a9955;
  border-radius: 7px; flex-shrink: 0;
}
.dz-file-icon.video-icon {
  background: rgba(26,114,232,0.10); color: #1a72e8;
}
.dz-file-info {
  flex: 1; min-width: 0;
  display: flex; flex-direction: column; gap: 2px;
}
.dz-file-name {
  font-family: 'JetBrains Mono', monospace;
  font-size: 12.5px; font-weight: 500;
  color: var(--text-primary, #1a1d23);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.dz-file-size { font-size: 11px; color: var(--text-muted, #8a96a8); }
.dz-check {
  width: 26px; height: 26px;
  display: flex; align-items: center; justify-content: center;
  background: rgba(26,153,85,0.12); color: #1a9955;
  border-radius: 50%; flex-shrink: 0;
}
.dz-remove {
  width: 26px; height: 26px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 5px; color: var(--text-muted, #8a96a8);
  border: none; background: none; cursor: pointer;
  transition: color 120ms ease, background 120ms ease;
  flex-shrink: 0;
}
.dz-remove:hover { color: #d93025; background: rgba(217,48,37,0.07); }

.format-note {
  display: flex; align-items: center; gap: 6px;
  margin-top: 10px; font-size: 11px; color: var(--text-muted, #8a96a8);
}
.format-note svg { flex-shrink: 0; }
.format-note strong { font-weight: 600; color: var(--text-secondary, #4a5568); }

/* Modal footer */
.modal-footer {
  display: flex; align-items: center; justify-content: flex-end;
  gap: 8px; padding: 14px 20px;
  border-top: 1px solid var(--border-subtle, rgba(0,0,0,0.07));
  background: var(--bg-panel, #f8f9fa);
}
.btn-cancel {
  padding: 8px 16px;
  font-family: 'Syne', sans-serif; font-size: 13px; font-weight: 600;
  color: var(--text-secondary, #4a5568);
  background: #fff;
  border: 1px solid var(--border-mid, rgba(0,0,0,0.12));
  border-radius: 5px; cursor: pointer;
  transition: background 120ms ease, color 120ms ease;
}
.btn-cancel:hover { background: var(--bg-void, #f0f1f3); color: var(--text-primary, #1a1d23); }
.btn-create {
  display: flex; align-items: center; gap: 6px;
  padding: 8px 18px;
  font-family: 'Syne', sans-serif; font-size: 13px; font-weight: 600;
  color: #fff; background: #e05c10;
  border: none; border-radius: 5px; cursor: pointer;
  transition: background 120ms ease, transform 60ms, opacity 120ms ease;
}
.btn-create:hover:not(:disabled) { background: #c44d08; }
.btn-create:active:not(:disabled) { transform: scale(0.97); }
.btn-create:disabled { opacity: 0.38; cursor: not-allowed; }
</style>