<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <v-app>
    <v-app-bar color="primary" density="compact">
      <v-app-bar-title>Farm + Tauri + Vue + Vuetify</v-app-bar-title>
    </v-app-bar>

    <v-main>
      <v-container class="d-flex flex-column align-center">
        <v-card class="mt-5 pt-5 px-5 pb-3" max-width="600" elevation="5">
          <v-card-title class="text-h4 text-center">
            欢迎使用 Farm + Tauri + Vue
          </v-card-title>

          <div class="d-flex justify-center my-4">
            <a href="https://farmfe.org/" target="_blank">
              <img src="/farm.png" class="logo farm" alt="Farm logo" />
            </a>
            <a href="https://tauri.app" target="_blank">
              <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
            </a>
            <a href="https://vuejs.org/" target="_blank">
              <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
            </a>
          </div>
          <v-card-text class="text-center">
            点击 Tauri、Farm 和 Vue 图标了解更多信息。
          </v-card-text>

          <v-card-actions class="flex-column pa-3 gap-3">
            <v-form @submit.prevent="greet" class="w-100">
              <v-row>
                <v-col cols="8">
                  <v-text-field
                    v-model="name"
                    label="请输入名称..."
                    variant="outlined"
                    hide-details
                  ></v-text-field>
                </v-col>
                <v-col cols="4">
                  <v-btn
                    type="submit"
                    color="primary"
                    block
                    height="56"
                  >
                    问候
                  </v-btn>
                </v-col>
              </v-row>
            </v-form>
            
            <v-alert
              v-if="greetMsg"
              type="success"
              variant="tonal"
              class="w-100 mt-3"
            >
              {{ greetMsg }}
            </v-alert>
          </v-card-actions>
        </v-card>
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.farm:hover {
  filter: drop-shadow(0 0 2em #ff7474);
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>