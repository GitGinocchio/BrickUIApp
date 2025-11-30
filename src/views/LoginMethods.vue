<template>
  <div class="main-container">
    <div class="icon">
      <img class="icon-image" src="../assets/images/notRegistered.svg"/>
    </div>

    <div class="login-register-container">
      <div class="description-container">
        <h2>Welcome to BrickUI!</h2>
        <div class="divider"></div>
        <p class="subtitle">
          Build your dream brick by brick!
        </p>
      </div>

      <div class="buttons-container">
        <n-button strong round size="large" style="background-color: #cb4153ff;" class="register-button" tertiary @click="goToRegister">
        Register
      </n-button>

      <div class="or">or</div>

      <n-button strong size="medium" style="background-color: #333;" class="login-button" tertiary  @click="goToLogin">
        Log In
      </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NButton } from 'naive-ui';
import { useRouter } from 'vue-router'

const router = useRouter()

const goToRegister = () => {
  router.push('/register')
}

const goToLogin = () => {
  router.push('/login')
}
</script>

<style scoped>
.main-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  align-items: center;
  justify-items: center;
  min-height: calc(100vh - 40px);
  padding: 2rem;
  box-sizing: border-box;
}

.icon,
.login-register-container {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-image {
  width: 100%;
  max-width: 550px;
  height: auto;
  display: block;
}

.login-register-container {
  flex-direction: column;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  gap: 1rem;
  max-width: 30rem;
}

.description-container {
  text-align: center;
}

.description-container h2 {
  font-size: 2.2rem;
  margin-bottom: 0.5rem;
}

.subtitle {
  font-size: 1rem;
  color: #aaa;
}

.buttons-container{
  display: flex;
  justify-content: center;
  align-items: center;
}

.register-button,
.login-button {
  width: 100%;
  padding: 1.2rem;
  font-size: 1.1rem;
  text-align: center;
  max-width: 9rem;
}

.divider {
  display: flex;
  align-items: center;
  text-align: center;
  border: 1px solid #777;
  opacity: 0.4;
  width: 100%;
  margin: 0.5rem 0;
  gap: 0.20rem;
}

.or{
  display: flex;
  align-items: center;
  text-align: center;
  color: #777;
  width: 100%;
  margin: 0.5rem 0;
  margin: 1rem;
}

</style>
