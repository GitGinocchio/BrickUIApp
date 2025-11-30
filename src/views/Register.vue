<template>
  <div class="register-container">
    <div class="register-content">
      <div class="welcome-message">
        <h1>Welcome UIBricker</h1>
        <svg xmlns="http://www.w3.org/2000/svg" color="#fff" width="50" height="50" viewBox="0 0 2836 2836">
          <g class="waving-arm" style="transform-origin: 1418px 1180px;">
            <path fill="currentColor" d="M726.6 795.1c80.5 270.5 375.8 634 669.3 640.6 231.1 5.1 485.5-5.9 700.7.6 236.4 7.1 345.9 130.4 341.8 364.9-1.2 231.6-0.9 343.9-0.1 414.6 1.3 117.7-0.1 180.3-131.3 182.3-592.6-4.4-1013.4 1.6-1013.4 1.6-144.7-11.1-142.4-67.3-142.6-184.2-0.3-208.3 0.2-325.9 0.2-325.9-1.4-112.5-37.2-137.1-125.4-192.4C712 1500.6 568.4 1306.2 410 886.5c-80-212 258.6-312.6 316.6-91.4z"/>
          </g>
          <circle fill="currentColor" cx="1697.73" cy="873.61" r="437.84"/>
        </svg>
      </div>

      <p class="little-descr">
        lovely to see you here! <br />
        Register and be part of our family
      </p>

      <!-- FORM NAIVE UI -->
      <div class="form-container">
        <NForm
          :model="form"
          :rules="rules"
          ref="formRef"
          label-placement="top"
          require-mark-placement="right-hanging"
        >
          <!-- USERNAME -->
          <NFormItem label="Username" path="username">
            <NInput
              v-model:value="form.username"
              placeholder="Enter your username"
              type="text"
              size="large"
            />
          </NFormItem>

          <!-- EMAIL -->
          <NFormItem label="Email" path="email">
            <NInput
              v-model:value="form.email"
              placeholder="Enter your email"
              type="text"
              size="large"
            />
          </NFormItem>

          <!-- PASSWORD -->
          <NFormItem label="Password" path="password">
            <NInput
              v-model:value="form.password"
              :type="showPassword ? 'text' : 'password'"
              placeholder="Enter your password"
              size="large"
            >
              <template #suffix>
                <button
                  type="button"
                  class="toggle-btn"
                  @click="showPassword = !showPassword"
                >
                  <span v-if="showPassword">
                    <!-- Occhi aperti -->
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="20" viewBox="0 0 48 24" fill="none"
                         stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M2 12s3-6 8-6 8 6 8 6-3 6-8 6-8-6-8-6z" />
                      <circle cx="10" cy="12" r="2.2" fill="white" />
                      <line x1="10" y1="6" x2="10" y2="3" />
                      <line x1="6" y1="7.5" x2="4" y2="5.5" />
                      <line x1="14" y1="7.5" x2="16" y2="5.5" />
                      <path d="M26 12s3-6 8-6 8 6 8 6-3 6-8 6-8-6-8-6z" />
                      <circle cx="34" cy="12" r="2.2" fill="white" />
                      <line x1="34" y1="6" x2="34" y2="3" />
                      <line x1="30" y1="7.5" x2="28" y2="5.5" />
                      <line x1="38" y1="7.5" x2="40" y2="5.5" />
                    </svg>
                  </span>
                  <span v-else>
                    <!-- Occhi chiusi -->
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="20" viewBox="0 0 48 24" fill="none"
                         stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M2 12q8-6 16 0" />
                      <line x1="10" y1="6" x2="10" y2="3" />
                      <line x1="6" y1="7.5" x2="4" y2="5.5" />
                      <line x1="14" y1="7.5" x2="16" y2="5.5" />
                      <path d="M26 12q8-6 16 0" />
                      <line x1="34" y1="6" x2="34" y2="3" />
                      <line x1="30" y1="7.5" x2="28" y2="5.5" />
                      <line x1="38" y1="7.5" x2="40" y2="5.5" />
                    </svg>
                  </span>
                </button>
              </template>
            </NInput>
          </NFormItem>

          <!-- CONFIRM PASSWORD -->
          <NFormItem label="Confirm Password" path="confirmPassword">
            <NInput
              v-model:value="form.confirmPassword"
              :type="showConfirmPassword ? 'text' : 'password'"
              placeholder="Confirm your password"
              size="large"
            >
              <template #suffix>
                <button
                  type="button"
                  class="toggle-btn"
                  @click="showConfirmPassword = !showConfirmPassword"
                >
                  <span v-if="showConfirmPassword">
                    <!-- Occhi aperti -->
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="20" viewBox="0 0 48 24" fill="none"
                         stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M2 12s3-6 8-6 8 6 8 6-3 6-8 6-8-6-8-6z" />
                      <circle cx="10" cy="12" r="2.2" fill="white" />
                      <line x1="10" y1="6" x2="10" y2="3" />
                      <line x1="6" y1="7.5" x2="4" y2="5.5" />
                      <line x1="14" y1="7.5" x2="16" y2="5.5" />
                      <path d="M26 12s3-6 8-6 8 6 8 6-3 6-8 6-8-6-8-6z" />
                      <circle cx="34" cy="12" r="2.2" fill="white" />
                      <line x1="34" y1="6" x2="34" y2="3" />
                      <line x1="30" y1="7.5" x2="28" y2="5.5" />
                      <line x1="38" y1="7.5" x2="40" y2="5.5" />
                    </svg>
                  </span>
                  <span v-else>
                    <!-- Occhi chiusi -->
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="20" viewBox="0 0 48 24" fill="none"
                         stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M2 12q8-6 16 0" />
                      <line x1="10" y1="6" x2="10" y2="3" />
                      <line x1="6" y1="7.5" x2="4" y2="5.5" />
                      <line x1="14" y1="7.5" x2="16" y2="5.5" />
                      <path d="M26 12q8-6 16 0" />
                      <line x1="34" y1="6" x2="34" y2="3" />
                      <line x1="30" y1="7.5" x2="28" y2="5.5" />
                      <line x1="38" y1="7.5" x2="40" y2="5.5" />
                    </svg>
                  </span>
                </button>
              </template>
            </NInput>
          </NFormItem>

          <!-- BOTTONE REGISTER -->
          <NButton
            type="primary"
            block
            strong
            size="large"
            @click="handleSubmit"
          >
            Register
          </NButton>
        </NForm>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, ref, Ref } from 'vue';
import { NForm, NFormItem, NInput, NButton, FormInst, FormRules, FormItemRule } from 'naive-ui';
import { fetch } from '@tauri-apps/plugin-http';
import { useRouter } from 'vue-router';
import { User } from 'interfaces/user';

const router = useRouter();

const form = ref({
  username: '',
  email: '',
  password: '',
  confirmPassword: ''
})


const formRef = ref<FormInst | null>(null);
const showPassword = ref(false);
const showConfirmPassword = ref(false);
const user = inject("user") as Ref<User|null>;

const validatePasswordMatch = (rule: FormItemRule, value: string): boolean | Error => {
  if (value !== form.value.password) {
    return new Error('Passwords do not match')
  }
  return true
}

const rules: FormRules = {
  username: [
    { required: true, message: 'Username is required', trigger: ['blur', 'input'] },
    { min: 3, message: 'Username must be at least 3 characters', trigger: 'input' },
    { max: 20, message: 'Username must be less than 20 characters', trigger: 'input' }
  ],
  email: [
    { required: true, message: 'Email is required', trigger: ['blur', 'input'] },
    { type: 'email', message: 'Invalid email format', trigger: ['blur', 'input'] }
  ],
  password: [
    { required: true, message: 'Password is required', trigger: ['blur', 'input'] },
    { min: 8, message: 'Password must be at least 8 characters', trigger: 'input' },
    { max: 128, message: 'Password too long', trigger: 'input' }
  ],
  confirmPassword: [
    { required: true, message: 'Please confirm your password', trigger: ['blur', 'input'] },
    { validator: validatePasswordMatch, trigger: ['blur', 'input'] }
  ]
}

const handleSubmit = async () => {
  try {
    await formRef.value?.validate()
    console.log('Valid form:', form.value)

    const payload ={
      username : form.value.username,
      email : form.value.email,
      password : form.value.password
    };

    // Trasformiamo tramite un encoder il payload in un array di variabili:
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));
    
    const response = await fetch("https://brickui.giulioo.workers.dev/api/auth/register/classic", {
      method: 'POST',
      body: body
    });

    const registerResponse = await response.json();

    switch(registerResponse.code){
      case 422:
        console.log("Credenziali non valide");
        break;
      case undefined:
        router.push('/user');
    }
  }
  catch (err) {
    console.warn('Invalid form:', err)
  }
}
</script>

<style scoped>
.register-container {
  border-radius: 20px;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background-color: #23232c;
  padding: 30px 35px;
  width: 90%;
  max-width: 420px;
  box-sizing: border-box;
}

.register-content {
  display: flex;
  flex-direction: column;
  color: white;
}

.welcome-message {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 10px;
}

.welcome-message h1 {
  font-size: 24px;
  margin: 0;
  font-weight: 600;
}

.little-descr {
  font-size: 14px;
  line-height: 1.5;
  color: #cfc9c9;
  opacity: 0.7;
  margin: 0 0 25px 0;
}

.form-container {
  width: 100%;
}

.form-container :deep(.n-form-item) {
  margin-bottom: 16px;
}

.form-container :deep(.n-form-item-label) {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 8px;
}

.form-container :deep(.n-input) {
  height: 48px;
}

.form-container :deep(.n-input .n-input__input-el) {
  padding-top: 7px;
}

.form-container :deep(.n-input .n-input-wrapper) {
  padding-left: 16px;
  padding-right: 16px;
}

.form-container .n-button {
  margin-top: 8px;
  height: 48px;
  font-size: 16px;
  font-weight: 600;
}

.toggle-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 4px;
  margin-right: -4px;
}

.toggle-btn svg {
  pointer-events: none;
  display: block;
}

@media (max-width: 768px) {
  .register-container {
    padding: 30px 25px;
    width: 95%;
  }
  
  .welcome-message h1 {
    font-size: 20px;
  }
  
  .welcome-message svg {
    width: 40px;
    height: 40px;
  }
}
</style>