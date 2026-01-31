<template>
  <div class="section">
    <n-card>
      <div class="profile-header">
        <div class="avatar-wrapper">
          <n-image
            class="avatar"
            src="https://placehold.net/avatar-3.svg"
            alt="User Photo"
            width="150"
            height="150"
            preview-disabled
          />
          <label class="change-photo">
            <input type="file" hidden />
            Cambia foto
          </label>
        </div>

        <div class="user-info">
          <h1 class="username">Username</h1>

          <ul class="stats">
            <li>0 Follower</li>
            <li>0 Seguiti</li>
            <li>0 Brick</li>
          </ul>
        </div>
      </div>

      <div class="actions">
        <n-button type="primary">Modifica profilo</n-button>
        <n-button secondary>Condividi profilo</n-button>
      </div>
    </n-card>
  </div>
  <div class="section">
    <n-card title="Bricks">
        <div class="bricks-wrapper">
            <n-button
            v-if="showArrows"
            quaternary
            class="arrow left"
            @click="scrollLeft"
            >
            </n-button>

            <div ref="scrollContainer" class="bricks-container">
            <div
              v-for="brick in bricks"
              :key="brick.id"
              class="brick-card"
            >
              {{ brick.title }}
            </div>
            </div>

            <n-button
            v-if="showArrows"
            quaternary
            class="arrow right"
            @click="scrollRight"
            >
            </n-button>
        </div>
    </n-card>
  </div>
  <div class="section">
    <n-card title="Favorite Bricks">
        <div class="bricks-wrapper">
            <n-button
              v-if="showArrows"
              quaternary
              class="arrow left"
              @click="scrollLeft"
            >
            </n-button>

            <div ref="scrollContainer" class="bricks-container">
            <div
              v-for="brick in bricks"
              :key="brick.id"
              class="brick-card"
            >
              {{ brick.title }}
            </div>
            </div>

            <n-button
              v-if="showArrows"
              quaternary
              class="arrow right"
              @click="scrollRight"
            >
            </n-button>
        </div>
    </n-card>
  </div>
</template>


<script setup lang="ts">
import { NImage, NButton, NCard } from 'naive-ui'
import { ref, onMounted } from 'vue'

const bricks = ref([
  { id: 1, title: 'Brick 1' },
  { id: 2, title: 'Brick 2' },
  { id: 3, title: 'Brick 3' },
  { id: 4, title: 'Brick 4' },
  { id: 5, title: 'Brick 5' }
])

const scrollContainer = ref<HTMLElement | null>(null)
const showArrows = ref(false)

const checkOverflow = () => {
  if (!scrollContainer.value) return
  showArrows.value =
    scrollContainer.value.scrollWidth >
    scrollContainer.value.clientWidth
}

const scrollLeft = () => {
  scrollContainer.value?.scrollBy({
    left: -200,
    behavior: 'smooth'
  })
}

const scrollRight = () => {
  scrollContainer.value?.scrollBy({
    left: 200,
    behavior: 'smooth'
  })
}

onMounted(() => {
  checkOverflow()
  window.addEventListener('resize', checkOverflow)
})
</script>



<style scoped>
.section {
  margin: 4rem;
}

.profile-header {
  display: flex;
  gap: 2rem;
  align-items: center;
}

.avatar-wrapper {
  position: relative;
  text-align: center;
}

.avatar {
  border-radius: 50%;
  cursor: pointer;
}

.change-photo {
  display: block;
  margin-top: 0.5rem;
  font-size: 14px;
  color: #4098fc;
  cursor: pointer;
}

.user-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.username {
  margin: 0;
}

.stats {
  display: flex;
  gap: 1rem;
  list-style: none;
  padding: 0;
  margin: 0;
}

.actions {
  margin-top: 2rem;
  display: flex;
  gap: 1rem;
  flex-direction: column;
}

.bricks-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.bricks-container {
  display: flex;
  gap: 1rem;
  overflow-x: auto;
  scroll-behavior: smooth;
  padding: 0.5rem 0;
}

/* Nasconde scrollbar */
.bricks-container::-webkit-scrollbar {
  display: none;
}

.brick-card {
  min-width: 160px;
  height: 120px;
  background: #f5f5f5;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

/* Frecce */
.arrow {
  position: absolute;
  z-index: 2;
}

.arrow.left {
  left: -8px;
}

.arrow.right {
  right: -8px;
}

.card {
  height: 160px;
  border-radius: 12px;
  background: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
}

</style>