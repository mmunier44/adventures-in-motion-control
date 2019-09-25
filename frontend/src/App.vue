<template>
  <div id="app" class="wrapper">
    <Sidebar :frequency="frequency" :tick_duration_us="tick_duration_us" />

    <b-card class="body" no-body>
      <b-tabs content-class="mt-3" card>
        <b-tab title="Controls">
          <b-card-text>I contain buttons and things</b-card-text>
        </b-tab>
        <b-tab title="G-Code Viewer" active>
          <GCodeViewer :text="program" />
        </b-tab>
        <b-tab title="Terminal">
          <b-card-text>I'm the communications monitor and let you see messages go back and forth.</b-card-text>
        </b-tab>
        <b-tab title="Configuration">
          <b-card-text>This is where you configure the machine.</b-card-text>
        </b-tab>
      </b-tabs>
    </b-card>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import Sidebar from "@/components/Sidebar.vue";
import GCodeViewer from "@/components/GCodeViewer.vue";
import * as wasm from "aimc_sim";

@Component({ components: { Sidebar, GCodeViewer } })
export default class App extends Vue {
  private app?: wasm.App;
  private animateToken = 0;
  public frequency = 0;
  public tick_duration_us = 0;
  public program = "Pretend I'm a\ng-code program";

  mounted() {
    // setup the world
    this.app = wasm.setup_world();

    // and schedule the animate() function to be called on the next tick
    this.animateToken = requestAnimationFrame(this.animate.bind(this));
  }

  beforeDestroy() {
    // make sure the animate method is cancelled when this component is removed
    // from the screen
    cancelAnimationFrame(this.animateToken);

    // don't forget to drop() our App
    if (this.app) {
      this.app.free();
      this.app = undefined;
    }
  }

  animate() {
    // schedule animate to be called again
    this.animateToken = requestAnimationFrame(this.animate.bind(this));

    if (this.app) {
      // poll the app to let it make progress
      wasm.poll(this.app, this);
    }
  }

  set_fps(frequency: number, tick_duration_ms: number) {
    this.frequency = Math.round(frequency * 10) / 10;
    this.tick_duration_us = Math.round(tick_duration_ms * 1000);
  }

  send_data(data: Uint8Array) {
    console.log(new TextDecoder("utf-8").decode(data));
    // TODO: actually handle the message...
  }
}
</script>

<style>
body {
  padding: 0;
  margin: 0;
}

.wrapper {
  display: flex;
  min-height: 100vh;
  flex-direction: row;
  align-items: stretch;
}

.body {
  flex: 1;
  margin: 30px;
}
</style>