<template>
  <div class="register-container">

    <div class="register-content">

      <div class="welcome-message">
        <h1>Welcome UIBricker</h1>
        <svg xmlns="http://www.w3.org/2000/svg" color="#fff" width="60" height="60" viewBox="0 0 2836 2836">
  <g class="waving-arm" style="transform-origin: 1418px 1180px;">
    <path fill="currentColor" d="M 726.617188 795.121094 C 807.160156 1065.660156 1102.410156 1429.160156 1395.878906 1435.691406 C 1627 1440.839844 1881.441406 1429.890625 2096.679688 1436.351562 C 2333.070312 1443.449219 2442.609375 1566.738281 2438.5 1801.160156 C 2437.269531 2032.800781 2437.539062 2145.070312 2438.339844 2215.792969 C 2439.648438 2333.515625 2438.28125 2396.183594 2306.988281 2398.226562 C 1714.449219 2393.847656 1293.640625 2399.820312 1293.640625 2399.820312 C 1148.929688 2388.652344 1151.238281 2332.441406 1151.078125 2215.59375 C 1150.789062 2007.3125 1151.339844 1889.734375 1151.339844 1889.734375 C 1149.910156 1777.21875 1114.078125 1752.609375 1025.96875 1697.390625 C 711.972656 1500.628906 568.355469 1306.21875 409.953125 886.519531 C 329.988281 674.550781 668.515625 573.890625 726.617188 795.121094 Z"/>
  </g>
  <circle fill="currentColor" cx="1697.73" cy="873.61" r="437.84"/></svg>
      </div>

      <p class="little-descr">lovely to see you here! <br>Register and be part of our family</p>

      <div class="form-container">
        <form>
          <input type="text" placeholder="Username" class="input-field" required/>

          <input type="email" placeholder="Email" class="input-field" required/>

          <input type="password" placeholder="Password" class="input-field" required/>

          <input type="password" placeholder="Confirm Password" class="input-field" required/>

          <NButton type="primary" class="register-button">Register</NButton>
        </form>
      </div>

    </div>

  </div>
</template>

<script setup lang="ts">
import { NButton } from 'naive-ui';

</script>

<style scoped>

.register-container { 
  border-radius: 20px;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background-color: #23232c;
  height: 75vh;
  width: 30vw;
  min-width: 400px;
}

.register-content{
  display: flex;
  flex-direction: column;
  margin: 20px 35px 0 35px;
  height: 100%;
  color: white;
}


.little-descr{
  font-size: 14px;
  margin-top: -20px;
  color: #cfc9c9;
  opacity: 0.56;
}

.welcome-message{
  display: flex;
  align-items: center;
  align-content: center;
  gap: 10px;
}

.form-container{
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

.form-container {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

form {
  display: flex;
  flex-direction: column;
  gap: 20px;
  position: relative;
  top: 8%;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: #e0e0e0;
  margin: 0;
}

.input-field {
  width: 100%;
  padding: 12px 16px;
  background-color: #2e2e38;
  border: 1px solid #3a3a44;
  border-radius: 8px;
  color: white;
  font-size: 14px;
  transition: all 0.3s ease;
  box-sizing: border-box;
}

.input-field::placeholder {
  color: #7a7a82;
}

.input-field:focus {
  outline: none;
  border-color: #5a7cf8;
  background-color: #32323d;
  box-shadow: 0 0 0 3px rgba(90, 124, 248, 0.1);
}

.input-field:hover {
  border-color: #4a4a54;
}

.register-button {
  margin-top: 10px;
  width: 100%;
  height: 44px;
  font-size: 15px;
  font-weight: 600;
  border-radius: 8px;
  cursor: pointer;
}

/* Scrollbar personalizzata */
.register-content::-webkit-scrollbar {
  width: 6px;
}

.register-content::-webkit-scrollbar-track {
  background: transparent;
}

.register-content::-webkit-scrollbar-thumb {
  background: #3a3a44;
  border-radius: 3px;
}

.register-content::-webkit-scrollbar-thumb:hover {
  background: #4a4a54;
}

</style>