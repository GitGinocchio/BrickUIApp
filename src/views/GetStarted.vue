<template>
  <div class="container">
    <Header :sections="sections" />
    <n-card class="card-container">
      <div class="reg-slogan">
        <!--<h1 class="title">{{ randomTitle }}</h1>-->
        <cite style="font-size: 1.25rem;" class="subtitle">
          lovely to see you here! 
          Login or Register and be part of our family
        </cite>
      </div>
      <div class="divider"></div>
      <n-tabs
        :active-name="activeTab"
        @update:value="(newTab) => activeTab = newTab"
        default-value="signup"
        size="large"
        animated
      >
        <n-tab-pane name="signup" tab="Sign Up">
          <n-form
            :model="form" 
            :rules="RegisterRules"
            ref="signupformRef"
            @keydown.enter.prevent="throttledRegister"
          >
            <n-form-item-row label="Email" path="email">
              <n-input v-model:value="form.email" placeholder="Email"/>
            </n-form-item-row>
            <n-form-item-row label="Password" path="password">
              <n-input
                v-model:value="form.password"
                :type="showSignupPassword ? 'text' : 'password'"
                placeholder="Password"
              >
                <template #suffix>
                  <button type="button" class="toggle-btn" @click.left.stop="showSignupPassword = !showSignupPassword">
                    <EyesOpened v-if="showSignupPassword" />
                    <EyesClosed v-else />
                  </button>
                </template>
              </n-input>
            </n-form-item-row>
            <n-form-item-row label="Confirm Password" path="confirmPassword">
              <n-input
                v-model:value="form.confirmPassword"
                :type="showSignupConfirmPassword ? 'text' : 'password'"
                placeholder="Confirm Password"
              >
                <template #suffix>
                  <button type="button" class="toggle-btn" @click.left.stop="showSignupConfirmPassword = !showSignupConfirmPassword">
                    <EyesOpened v-if="showSignupConfirmPassword" />
                    <EyesClosed v-else />
                  </button>
                </template>
              </n-input>
            </n-form-item-row>

            <NButton class="form-button" type="primary" :loading="loading" block strong size="large" @click="throttledRegister">
              Register
            </NButton>
          </n-form>
        </n-tab-pane>

        <n-tab-pane name="signin" tab="Sign In">
          <n-form
            :model="form"
            :rules="LoginRules"
            ref="signinFormRef"
            @keydown.enter.prevent="throttledLogin"
          >
            <n-form-item-row label="Email" path="email">
              <n-input v-model:value="form.email" placeholder="Email"/>
            </n-form-item-row>
            <n-form-item-row label="Password" path="password">
              <n-input
                v-model:value="form.password"
                :type="showSigninPassword ? 'text' : 'password'"
                placeholder="Password"
              >
                <template #suffix>
                  <button class="toggle-btn" type="button" @click.left.stop="showSigninPassword = !showSigninPassword">
                    <EyesOpened v-if="showSigninPassword" />
                    <EyesClosed v-else />
                  </button>
                </template>
              </n-input>
            </n-form-item-row>
            <NButton class="form-button" type="primary" :loading="loading" block strong size="large" @click="throttledLogin">
              Login
            </NButton>
          </n-form>
        </n-tab-pane>
      </n-tabs>
    </n-card>

    <div class="alert">
      <n-alert
        v-if="showAlert"
        :type="alertType"
        :title="alertTitle"
        closable
        @close="showAlert = false"
      >
        {{ alertMessage }}
        <div class="resend-button" v-if="showAlertResendEmailBtn">
          <n-button @click="debouncedResendEmail">
            <span v-if="cooldown > 0">({{ cooldown }}s)</span>
            Resend email
          </n-button>
        </div>
      </n-alert>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NForm, NInput, NButton, FormInst, FormRules, FormItemRule, NCard, NTabs, NTabPane, NFormItemRow, NAlert } from 'naive-ui'
import EyesClosed from '../components/icons/EyesClosed.vue'
import EyesOpened from '../components/icons/EyesOpened.vue'
import { fetch } from '@tauri-apps/plugin-http';
import { useI18n } from "vue-i18n";
import { UserIcon } from 'lucide-vue-next';
import { useRouter } from "vue-router";
import Header from '../components/Header.vue';
import { debounce, throttle } from '../utils/misc'
import { invoke } from '@tauri-apps/api/core';

const { t } = useI18n();
const router = useRouter();
const API_URL = import.meta.env.VITE_API_URL;

const form = ref({ email: '', password: '', confirmPassword: '' })
const signupformRef = ref<FormInst | null>(null);
const signinFormRef = ref<FormInst | null>(null);
const showSigninPassword = ref(false);
const showSignupPassword = ref(false);
const showSignupConfirmPassword = ref(false);
const activeTab = ref('signup');
const loading = ref<boolean>(false);

const cooldown = ref(0); // secondi rimanenti per il cooldown
const cooldownInterval = ref<number | null>(null);

const showAlert = ref(false);
const showAlertResendEmailBtn = ref(false);
const alertType = ref<"warning" | "error" | "success" | "default" | "info">('error');
const alertTitle = ref<string>('');
const alertMessage = ref<string>('');

const sections = computed(() => [{ icon: UserIcon, label: t('User') }]);

const LoginRules: FormRules = {
  email: [
    { required: true, message: 'Email is required', trigger: ['blur', 'input'] },
    { type: 'email', message: 'Invalid email format', trigger: ['blur', 'input'] }
  ],
  password: [
    { required: true, message: 'Password is required', trigger: ['blur', 'input'] },
    { min: 8, message: 'Password must be at least 8 characters', trigger: 'input' },
    { max: 128, message: 'Password too long', trigger: 'input' }
  ]
}

const RegisterRules: FormRules = {
  email: LoginRules.email,
  password: LoginRules.password,
  confirmPassword: [
    { required: true, message: 'Please confirm your password', trigger: ['blur', 'input'] },
    { validator: validatePasswordMatch, trigger: ['blur', 'input'] }
  ]
}

// Debounce combinato
const debouncedResendEmail = debounce(() => handleResendEmail(), 500);

const throttledLogin = throttle(() => handleLogin(), 5000);
const throttledRegister = throttle(() => handleRegister(), 5000);

const titles = ["Let’s Begin", "Welcome UIBricker!", "Join Us", "Register", "Sign Up", "We’re happy to have you!", "Become a UIBricker", "Create Your Account"];
const randomTitle = ref("");
onMounted(() => { randomTitle.value = titles[Math.floor(Math.random() * titles.length)]; });

async function handleRegister() {
  if (loading.value) return;
  try {
    console.log("Sending register request...");
    showAlertResendEmailBtn.value = false;
    loading.value = true;
    await signupformRef.value?.validate().catch((warnings) => {
      throw { code: -1, msg: warnings[0][0].message };
    });
    const payload = { email: form.value.email, password: form.value.password };
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));
    const response = await fetch(`${API_URL}/api/auth/register/classic`, {
      connectTimeout: 10000,
      method: 'POST', 
      body, 
      headers: { 
        "User-Agent": "BrickUIApp/1.0" 
      } 
    });

    let registerResponse: { msg: string, code: number } = { msg: 'Something went wrong when sending the request', code: null };
    try { 
      registerResponse = JSON.parse(await response.text()); 
    } 
    catch {
      throw { 
        code: registerResponse.code ?? response.status, 
        message: registerResponse.msg ?? response.statusText 
      };
    }

    if (registerResponse.code === 200 || (response as any).ok) {
      alertTitle.value = 'Successfully registered!';
      alertMessage.value = `We've sent a confirmation email to ${payload.email}.\nClick the link to activate your account.`;
      alertType.value = 'success';
      showAlert.value = true;
      showAlertResendEmailBtn.value = true;
      startCooldown(60);
      activeTab.value = 'signin';
      return;
    }

    throw registerResponse;
  } catch (err: any) {
    alertType.value = 'error';
    alertMessage.value = err.msg || "An unexpected error occurred";
    showAlert.value = true;

    console.error(err);

    switch (err.code) {
      case 500: alertTitle.value = 'Internal Server Error'; break;
      case 400: alertTitle.value = 'Validation Failed'; break;
      case 422: alertTitle.value = 'Email Exists'; break;
      case 409: alertTitle.value = 'Account Already Exists'; break;
      case -1: alertTitle.value = 'Invalid Input'; break;
      default: alertTitle.value = err.code ? `Login Error: ${err.code}` : 'Unexpected Error'; break;
    }
  } 
  finally { 
    loading.value = false; 
  }
}

async function handleLogin() {
  if (loading.value) return;
  try {
    console.log("Sending login request...");
    showAlertResendEmailBtn.value = false;
    loading.value = true;
    await signinFormRef.value?.validate().catch((warnings) => {
      throw { code: -1, msg: warnings[0][0].message };
    });
    const payload = { email: form.value.email, password: form.value.password };
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));
    const response = await fetch(`${API_URL}/api/auth/login`, {
      connectTimeout: 10000,
      method: 'POST', 
      body, 
      headers: { 
        "User-Agent": "BrickUIApp/1.0" 
      } 
    });

    let loginResponse: { msg: string, code: number } = { msg: 'Something went wrong when sending the request', code: null };
    try { 
      loginResponse = JSON.parse(await response.text()); 
    } 
    catch { 
      throw { 
        code: loginResponse.code ?? response.status, 
        message: loginResponse.msg ?? response.statusText 
      }; 
    }

    if(loginResponse.code === 200 || response.ok) {
      console.log(loginResponse);
      await invoke("auth_complete_login", { authResponse: loginResponse});
      router.push('/user');
      return; 
    }
    throw loginResponse;
  } catch (err: any) {
    alertType.value = 'error';
    alertMessage.value = err.msg;
    showAlert.value = true;

    console.error(err);

    switch (err.code) {
      case -1: alertTitle.value = `Invalid input`; break;
      case 429:
        showAlertResendEmailBtn.value = true;
        break;
      case 400:
        switch (err.error_code) {
          case "invalid_credentials": 
            alertTitle.value = `Invalid Credentials`; 
            break;
          case "email_not_confirmed":
            alertTitle.value = `Email not confirmed`;
            alertMessage.value = "Your email hasn’t been confirmed yet. Check your inbox for the verification link to complete sign-in.";
            showAlertResendEmailBtn.value = true;
            break;
        }
        break;
      default: alertTitle.value = `Login Error ${err.code}: Unexpected Error`; break;
    }
  } 
  finally { 
    loading.value = false; 
  }
}

function startCooldown(seconds: number) {
  cooldown.value = seconds;

  if (cooldownInterval.value) clearInterval(cooldownInterval.value);

  cooldownInterval.value = window.setInterval(() => {
    cooldown.value -= 1;
    if (cooldown.value <= 0) {
      clearInterval(cooldownInterval.value!);
      cooldownInterval.value = null;
    }
  }, 1000);
}

async function handleResendEmail() {
  if (cooldown.value > 0) return; // blocca se in cooldown

  try {
    const payload = { email: form.value.email };
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));
    const response = await fetch(`${API_URL}/api/auth/resend`, {
      connectTimeout: 10000,
      method: 'POST',
      body,
      headers: { "User-Agent": "BrickUIApp/1.0" }
    });

    let resendResponse: { msg: string, code: number, cooldown?: number } = { msg: 'Something went wrong', code: null };
    try { 
      resendResponse = JSON.parse(await response.text()) 
    }
    catch { 
      throw { 
        code: resendResponse.code ?? response.status, 
        message: resendResponse.msg ?? response.statusText 
      };
    }

    if (resendResponse.code === 200 || response.ok) {
      alertTitle.value = 'Email Sent!';
      alertMessage.value = `We've sent a confirmation email to ${payload.email}.`;
      showAlertResendEmailBtn.value = true;
      alertType.value = 'success';
      showAlert.value = true;
      startCooldown(60); // 60 secondi di cooldown
      return;
    }

    // Se arriva 429 o altri errori temporanei, parte solo il countdown
    if (resendResponse.code === 429) {
      startCooldown(resendResponse.cooldown ?? 120);
      return;
    }

    throw resendResponse;
  } catch (err: any) {
    // errori generici
    alertType.value = 'error';
    alertTitle.value = err.code ? `Error ${err.code}` : 'Unexpected Error';
    alertMessage.value = err.msg || 'An unexpected error occurred';
    showAlertResendEmailBtn.value = false;
    showAlert.value = true;

    console.error(err);
  }
}

function validatePasswordMatch(_rule: FormItemRule, value: string): boolean | Error {
  if (value !== form.value.password) {
    return new Error('Passwords do not match')
  }
  return true
}
</script>


<style scoped>
.container {
  display: flex;
  flex-direction: column;
  padding: 16px;
  gap: 1rem;
}

.reg-slogan{
  color: white;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.title{
  font-size: 2.4rem;
  margin: 0 0 0.5rem 0;
}

.divider {
  height: 0.07rem;
  background: linear-gradient(to left, transparent, #cb4153ff);
  margin-bottom: 0.75rem;
  width: 100%;
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

.alert{
  display: flex;
  justify-content: flex-end;
  position: absolute;
  bottom: 1rem;
  right: 1rem;
}

.resend-button {
  margin-top: 0.5rem;
}

.resend-button .n-button span {
  margin-right: 0.3rem;
}

.form-button {
  margin-top: 1rem;
}
</style>