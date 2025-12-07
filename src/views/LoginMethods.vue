<template>
  <div class="container">
    <Header :sections="sections"></Header>
    <n-card class="card-container">
      <div class="description-container">
        <h2>Welcome to BrickUI!</h2>
        <div class="divider"></div>
        <p class="subtitle">
          Build your path by placing new bricks or continue from <br> where you left off.
        </p>
      </div>

      <div class="buttons-container">
        <n-button strong round size="large" style="background-color: #cb4153ff;" class="register-button" tertiary @click="goToRegister">
        Register
        </n-button>

        <n-button strong size="medium" style="background-color: #333;" class="login-button" tertiary  @click="goToLogin">
          Log In
        </n-button>
      </div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { NButton, NCard } from 'naive-ui';
import { useRouter } from 'vue-router'
import Header from '../components/Header.vue';
import { computed } from 'vue';
import { User } from 'lucide-vue-next';
import { useI18n } from 'vue-i18n';

const router = useRouter()
const { t } = useI18n();

const goToRegister = () => {
  router.push('/register')
}

const goToLogin = () => {
  router.push('/login')
}
const sections = computed(() => {
  return [
    { icon: User, label: t('user') }, 
  ]
});
</script>

<style scoped>

.container {
  display: flex;
  flex-direction: column;
  padding: 16px;
  gap: 1rem;
}

.card-container {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 55rem;
}


.description-container {
  text-align: center;
  margin-bottom: 1rem;
}

.description-container h2 {
  font-size: 2.4rem;
  margin-bottom: 0.7rem;
  font-weight: 700;
  color: #fff;
  letter-spacing: 0.5px;
}

.subtitle {
  font-size: 1.1rem;
  color: #cfcfcf;
  line-height: 1.5rem;
}

.divider {
  height: 1px;
  background: linear-gradient(to right, transparent, #555, transparent);
  margin: 1rem auto;
  width: 80%;
}

.buttons-container {
  display: flex;
  flex-direction: column;
  gap: 1.2rem;
  align-items: center;
}

/* Register button */
.register-button {
  width: 100%;
  max-width: 11rem;
  padding: 1.2rem;
  font-size: 1.2rem;
  border-radius: 30px;
  transition: all 0.25s ease;
  background-color: #cb4153ff !important;
  color: #fff;
}

.register-button:hover {
  transform: translateY(-2px);
}

/* Login button */
.login-button {
  width: 100%;
  max-width: 11rem;
  padding: 1.15rem;
  font-size: 1.1rem;
  border-radius: 30px;
  transition: all 0.25s ease;
  background-color: #333 !important;
  color: #fff;
}

.login-button:hover {
  background-color: #444 !important;
  transform: translateY(-2px);
}

</style>