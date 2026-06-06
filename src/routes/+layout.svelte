<script>
  import { page } from '$app/stores';
  import './layout.css';

  // ── State ─────────────────────────────────────────────────
  let activeMenu = 'Timeline';

  const menuItems = [];

  const navItems = [
    { id: 'PROJECT',   label: 'Project',   href: '/projects',  icon: 'project' },
    { id: 'TIMELINE',  label: 'Timeline',  href: '/timeline',  icon: 'timeline' },
    
  ];

  $: activePath = $page.url.pathname;
</script>

<div class="app-shell">

  <!-- ═══════════════════════════════════════════════════════
       TOP NAV BAR
       ═══════════════════════════════════════════════════════ -->
  <nav class="top-nav">

    <!-- Brand -->
    <div class="brand">
      <div class="brand-icon">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M6 3h12l4 6-10 13L2 9z"/>
        </svg>
      </div>
      Snip<span>Snap</span>
    </div>

    <!-- Editor menu items -->
    <div class="nav-items" style="margin-right: 8px;">
      {#each menuItems as item}
        <button
          class="nav-item {activeMenu === item ? 'active' : ''}"
          style="text-transform: none; font-size: 12.5px; letter-spacing: 0; padding: 5px 10px;"
          on:click={() => activeMenu = item}
        >{item}</button>
      {/each}
    </div>

    <div class="nav-divider"></div>

    <!-- Section nav items (formerly sidebar) -->
    <div class="nav-items">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-item {activePath.startsWith(item.href) ? 'active' : ''}"
        >
          {#if item.icon === 'project'}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/>
            </svg>
          {:else if item.icon === 'timeline'}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="8" width="18" height="3" rx="1"/><rect x="3" y="13" width="13" height="3" rx="1"/><line x1="3" y1="5" x2="21" y2="5"/>
            </svg>
          {:else if item.icon === 'media'}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="2" width="20" height="20" rx="2.18"/><line x1="7" y1="2" x2="7" y2="22"/><line x1="17" y1="2" x2="17" y2="22"/><line x1="2" y1="12" x2="22" y2="12"/>
            </svg>
          {:else if item.icon === 'nodes'}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="5" cy="12" r="2"/><circle cx="19" cy="5" r="2"/><circle cx="19" cy="19" r="2"/>
              <line x1="7" y1="12" x2="17" y2="6"/><line x1="7" y1="12" x2="17" y2="18"/>
            </svg>
          {:else if item.icon === 'inspector'}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="4" y1="6" x2="20" y2="6"/><line x1="4" y1="12" x2="14" y2="12"/><line x1="4" y1="18" x2="10" y2="18"/>
            </svg>
          {/if}
          {item.label}
        </a>
      {/each}
    </div>

    <!-- Right: search + actions -->
    <div class="nav-right">
      <div class="search-bar">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input type="text" placeholder="Search assets..." />
      </div>

      <!-- Sync -->
      <button class="nav-sync">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="23 4 23 10 17 10"/><polyline points="1 20 1 14 7 14"/>
          <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
        </svg>
        Sync
      </button>

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

      <!-- Export -->
      <button class="nav-export">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
        Export
      </button>
    </div>
  </nav>

  <!-- ═══════════════════════════════════════════════════════
       PAGE SLOT
       ═══════════════════════════════════════════════════════ -->
  <div class="page-content">
    <slot />
  </div>

</div>