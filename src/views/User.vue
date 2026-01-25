<template>
  <div class="container">
    <Header :sections="headerSections">
      <template #actions>

      </template>
    </Header>

    <n-card>
      <div class="card-content">
        <n-image
          class="avatar"
          src="https://placehold.net/avatar-3.svg"
          alt="User Photo"
          width="125"
          height="125"
          :show-toolbar="false"
          :preview-disabled="true"
        >
        </n-image>

        <div class="user-info">
          <h2>{{ user?.display_name }}</h2>
          <p>abcd</p>
        </div>
      </div>
    </n-card>

  </div>
</template>

<script setup lang="ts">
import { User } from 'lucide-vue-next';
import Header from '../components/Header.vue';
import { NCard, NImage } from 'naive-ui';
import { h, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';


const headerSections = [
  { defaultIcon: () => h(User), label: 'User' }
]

const user = ref<any>();

onMounted(async () => {
  user.value = await invoke("users_get_me");
});

</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  padding: 1rem;
  gap: 1rem;
}

.card-content {
  display: flex;
  flex-direction: row;
  gap: 1rem;
}

.user-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.user-info h2 {
  margin: 0;
}

</style>