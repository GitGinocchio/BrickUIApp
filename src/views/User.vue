<template>
  <div class="container">
    <Header :sections="headerSections">
      <template #actions>
        <NButton circle tertiary><Settings2/></NButton>
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
        />

        <div class="user-info">
          <div class="field">
            <NSkeleton text v-if="loading" style="display: block; width: 30vw" size="medium" />
            <h2 v-else>{{ user?.display_name }}</h2>
            <NButton @click="" text circle size="medium">
              <Pencil v-if="true" :size="16" />
              <PencilOff v-else :size="16" />
            </NButton>
          </div>
          <div class="field">
            <NSkeleton text v-if="loading" style="display: block; width: 30vw" size="medium" />
            <h2 v-else>{{ user?.display_name }}</h2>
            <NButton @click="" text circle size="medium">
              <Pencil v-if="true" :size="16" />
              <PencilOff v-else :size="16" />
            </NButton>
          </div>

          <p>abcd</p>
        </div>
      </div>
    </n-card>

  </div>
</template>

<script setup lang="ts">
import { User, Pencil, PencilOff, Settings2 } from 'lucide-vue-next';
import Header from '../components/Header.vue';
import { NCard, NImage, NSkeleton, NButton } from 'naive-ui';
import { h, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';


const headerSections = [
  { defaultIcon: () => h(User), label: 'User' }
]

const loading = ref<boolean>(true);
const user = ref<any>();

onMounted(async () => {
  user.value = await invoke("users_get_me");
  console.log(user.value);
  //loading.value = false;
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
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.user-info .field {
  display: flex;
  flex-direction: row;
  gap: 1rem;
}

.user-info h2 {
  margin: 0;
}

</style>