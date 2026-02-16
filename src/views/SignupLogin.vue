<template>
  <div class="container">
    <Header :sections="sections" />
    <n-card class="card-container">
      <div class="reg-slogan">
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
        <!-- Sign Up Tab -->
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

        <!-- Sign In Tab -->
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, h } from 'vue'
import { NForm, NInput, NButton, FormInst, FormRules, FormItemRule, NCard, NTabs, NTabPane, NFormItemRow, useNotification } from 'naive-ui'
import EyesClosed from '../components/icons/EyesClosed.vue'
import EyesOpened from '../components/icons/EyesOpened.vue'
import { UserIcon } from 'lucide-vue-next';
import { useRouter } from "vue-router";
import Header from '../components/Header.vue';
import { debounce, throttle } from '../utils/misc'
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const router = useRouter();
const notify = useNotification();

const form = ref({ email: '', password: '', confirmPassword: '' })
const signupformRef = ref<FormInst | null>(null);
const signinFormRef = ref<FormInst | null>(null);

const showSigninPassword = ref(false);
const showSignupPassword = ref(false);
const showSignupConfirmPassword = ref(false);

const activeTab = ref('signup');
const loading = ref(false);
const cooldown = ref(0);
const cooldownInterval = ref<number | null>(null);

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

const debouncedResendEmail = debounce(() => handleResendEmail(), 500);
const throttledLogin = throttle(() => handleLogin(), 5000);
const throttledRegister = throttle(() => handleRegister(), 5000);

const titles = ["Let’s Begin", "Welcome UIBricker!", "Join Us", "Register", "Sign Up", "We’re happy to have you!", "Become a UIBricker", "Create Your Account"];
const randomTitle = ref("");
onMounted(() => { randomTitle.value = titles[Math.floor(Math.random() * titles.length)]; });

function showNotification(
  type: 'success'|'error'|'warning'|'info'|'default',
  title: string,
  message: string,
  showResendBtn = false
) {
  notify[type]({
    title,
    description: () => {
      const children = [
        h('p', { style: { fontSize: '1rem', margin: '0 0 0.5rem 0' } }, message)
      ];

      if (!showResendBtn) {
        return h('div', children);
      }

      children.push(
        h(NButton, 
          {
            size: 'small', 
            onClick: debouncedResendEmail
          },
          () => cooldown.value > 0 ? `(${cooldown.value}s) Resend email` : 'Resend email'
        )
      );

      return h('div', children);
    },
    duration: 0
  });
}

async function handleRegister() {
  if (loading.value) return;
  loading.value = true;
  try {
    await signupformRef.value?.validate().catch((warnings) => {
      throw { code: -1, msg: warnings[0][0].message };
    });

    const registerResponse: { status: string } = await invoke("auth_register", { 
      email: form.value.email, 
      password: form.value.password 
    });

    if (registerResponse.status === 'success') {
      showNotification(
        'success',
        'Successfully registered!',
        `We've sent a confirmation email to ${form.value.email}.\nClick the link to activate your account.`,
        true
      )
      startCooldown(60);
      activeTab.value = 'signin';
      return;
    }

    throw registerResponse;
  } catch (err: any) {
    showNotification('error', err.code != -1 ? `Error ${err.code}` : 'Invalid Input', err.msg || 'An unexpected error occurred');
    console.error(err);
  } finally {
    loading.value = false;
  }
}

async function handleLogin() {
  if (loading.value) return;
  loading.value = true;
  try {
    await signinFormRef.value?.validate().catch((warnings) => {
      throw { code: -1, msg: warnings[0][0].message };
    });

    const loginResponse: { status: string } = await invoke("auth_login", { 
      email: form.value.email, 
      password: form.value.password 
    });

    if (loginResponse.status === 'success') {
      router.push('/user');
      return;
    }

    throw loginResponse;
  } catch (err: any) {
    showNotification(
      'error', 
      err.code ? `Login Error: ${err.code}` : 'Invalid Input', 
      err.msg || 'An unexpected error occurred',
      err.error_code == 'email_not_confirmed'
    );
    console.error(err);
  } finally {
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
  if (cooldown.value > 0) return;

  try {
    const resendResponse: { status: string, code: number, cooldown?: number } = await invoke("auth_resend_email", { email: form.value.email });

    if (resendResponse.status === 'success') {
      showNotification('success', 'Email Sent!', `We've sent a confirmation email to ${form.value.email}.`, true);
      startCooldown(60);
      return;
    }

    if (resendResponse.status === 'error' && resendResponse.code === 429) {
      startCooldown(resendResponse.cooldown ?? 120);
      return;
    }

    throw resendResponse;
  } catch (err: any) {
    showNotification('error', err.code ? `Error ${err.code}` : 'Unexpected Error', err.msg || 'An unexpected error occurred');
    console.error(err);
  }
}

function validatePasswordMatch(_rule: FormItemRule, value: string): boolean | Error {
  if (value !== form.value.password) return new Error('Passwords do not match')
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

.form-button {
  margin-top: 1rem;
}
</style>
