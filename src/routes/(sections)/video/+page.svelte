<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/ui/button';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { mkdir, writeFile, BaseDirectory } from '@tauri-apps/plugin-fs';
  import { join, appDataDir } from '@tauri-apps/api/path';
  import { footerActionsStore } from '$lib';

  let file: File | null = null;
  let videoUrl: string | null = null;
  let processing = false;
  let message = '';

  function bufToHex(buf: ArrayBuffer) {
    const bytes = new Uint8Array(buf);
    const hex: string[] = [];
    for (let i = 0; i < bytes.length; i++) {
      const h = bytes[i].toString(16).padStart(2, '0');
      hex.push(h);
    }
    return hex.join('');
  }

  async function handleFileChange(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      file = input.files[0];
      videoUrl = URL.createObjectURL(file);
    }
  }

  async function cutAndExport() {
    if (!file) return;

    processing = true;
    message = '';
    try {
      // Write temp file under the app-scoped AppData directory
      const bytes = new Uint8Array(await file.arrayBuffer());
      const hashBuf = await crypto.subtle.digest('SHA-256', bytes);
      const hashHex = bufToHex(hashBuf);
      const short = hashHex.slice(0, 12);

      await mkdir('tmp', { baseDir: BaseDirectory.AppData, recursive: true });
      const filename = `video_cut_input_${short}.mp4`;
      await writeFile(`tmp/${filename}`, bytes, {
        baseDir: BaseDirectory.AppData
      });

      // Absolute path for the backend (GStreamer needs a real path)
      const appDir = await appDataDir();
      const tempPath = await join(appDir, 'tmp', filename);

      // Ask for output location
      const suggested = file.name.replace(/\.mp4$/i, '_cut.mp4');
      const outputPath = await save({
        defaultPath: suggested,
        filters: [{ name: 'MP4 Video', extensions: ['mp4'] }]
      });
      if (!outputPath) {
        message = 'Export cancelled';
        processing = false;
        return;
      }

      // Call Tauri backend
      const out = await invoke<string>('cut_video_in_half', {
        inputPath: tempPath,
        outputPath
      });
      message = `Exported: ${out}`;
    } catch (e) {
      message = `Error: ${e}`;
    } finally {
      processing = false;
    }
  }

  footerActionsStore.set([
    {
      type: 'button',
      label: 'Timeline',
      onClick: () => alert('/timeline'),
      active: true
    },
    {
      type: 'button',
      label: 'Audio',
      onClick: () => alert('/audio')
    },
    {
      type: 'button',
      label: 'Color Grade',
      onClick: () => alert('/color-grade')
    },
    {
      type: 'button',
      label: 'Finalize',
      onClick: () => alert('/finalize')
    }
  ]);
</script>

<main class="container">
  <Button onclick={() => goto('/')}>Home</Button>
  <h2>Video Cut Prototype</h2>
  <input type="file" accept="video/mp4" on:change={handleFileChange} />
  {#if videoUrl}
    <video src={videoUrl} controls style="width: 300px; margin-top: 1em;" />
  {/if}
  <Button onclick={cutAndExport} disabled={!file || processing}>
    {processing ? 'Processing...' : 'Cut & Export'}
  </Button>
  <p>{message}</p>
</main>
