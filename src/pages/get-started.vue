<template>
  <div class="container">
    <Header :sections="sections" />

    <div class="card-container bg-neutral-900/50 rounded-lg p-6">
      <div class="reg-slogan mb-4">
        <cite style="font-size: 1.25rem;" class="subtitle">
                  {{ t('get_started.slogan_line1') }}
                  {{ t('get_started.slogan_line2') }}
        </cite>
      </div>

      <div class="divider mb-6" />

      <UTabs :items="tabItems" v-model:active="activeTab" class="w-full">
        <template #signup>
          <div @keydown.enter.prevent="throttledRegister">
          <UFormField :label="t('labels.email')">
            <UInput v-model="form.email" :placeholder="t('placeholders.email')" />
            </UFormField>

            <UFormField :label="t('labels.password')">
              <UInput v-model="form.password" :type="showSignupPassword ? 'text' : 'password'" />
            </UFormField>

            <UFormField :label="t('labels.confirm_password')">
              <UInput v-model="form.confirmPassword" :type="showSignupConfirmPassword ? 'text' : 'password'" />
            </UFormField>

            <div class="mt-4">
              <UButton color="primary" variant="solid" :disabled="loading" @click="throttledRegister">{{ t('actions.register') }}</UButton>
            </div>
          </div>
        </template>

        <template #signin>
          <div @keydown.enter.prevent="throttledLogin">
          <UFormField :label="t('labels.email')">
            <UInput v-model="form.email" :placeholder="t('placeholders.email')" />
            </UFormField>

            <UFormField :label="t('labels.password')">
              <UInput v-model="form.password" :type="showSigninPassword ? 'text' : 'password'" />
            </UFormField>

            <div class="mt-4">
              <UButton color="primary" variant="solid" :disabled="loading" @click="throttledLogin">{{ t('actions.login') }}</UButton>
            </div>
          </div>
        </template>
      </UTabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import Header from '#components/Header.vue';
import { debounce, throttle } from '#utils/misc';
import { invoke } from '@tauri-apps/api/core';

const { t } = useI18n();
const router = useRouter();
const toast = useToast();

const form = ref({ email: '', password: '', confirmPassword: '' });
const showSigninPassword = ref(false);
const showSignupPassword = ref(false);
const showSignupConfirmPassword = ref(false);
const activeTab = ref('signup');
const loading = ref(false);
const cooldown = ref(0);
const cooldownInterval = ref<number | null>(null);

const sections = computed(() => [{ icon: null, label: t('User') }]);

const tabItems = [
  { label: t('auth.signup'), slot: 'signup' },
  { label: t('auth.signin'), slot: 'signin' }
];

function isEmail(v: string) {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v);
}

function validateWithRules(rules: Record<string, any[]>, model: Record<string, any>) {
  const errors: string[] = [];
  for (const key in rules) {
    const fieldRules = rules[key];
    const value = model[key];
    for (const r of fieldRules) {
      if (r.required && (value === undefined || value === null || value === '')) {
        errors.push(r.message || `${key} is required`);
        break;
      }
      if (r.type === 'email' && value && !isEmail(value)) {
        errors.push(r.message || `${key} is not a valid email`);
        break;
      }
      if (r.min !== undefined && typeof value === 'string' && value.length < r.min) {
        errors.push(r.message || `${key} is too short`);
        break;
      }
      if (r.max !== undefined && typeof value === 'string' && value.length > r.max) {
        errors.push(r.message || `${key} is too long`);
        break;
      }
      if (r.validator && typeof r.validator === 'function') {
        const res = r.validator(r, value);
        if (res instanceof Error) {
          errors.push(res.message || 'Validation error');
          break;
        }
        if (res === false) {
          errors.push(r.message || 'Validation failed');
          break;
        }
      }
    }
  }
  if (errors.length) return Promise.reject(errors);
  return Promise.resolve();
}

const LoginRules = {
  email: [
    { required: true, message: t('validation.email_required') },
        { type: 'email', message: t('validation.invalid_email') }
  ],
  password: [
    { required: true, message: t('validation.password_required') },
        { min: 8, message: t('validation.password_min') },
        { max: 128, message: t('validation.password_max') }
  ]
};

function validatePasswordMatch(_rule: any, value: string) {
  if (value !== form.value.password) return new Error(t('validation.passwords_do_not_match'));
  return true;
}

const RegisterRules = {
  email: LoginRules.email,
  password: LoginRules.password,
  confirmPassword: [
    { required: true, message: t('validation.confirm_password') },
    { validator: validatePasswordMatch }
  ]
};

const debouncedResendEmail = debounce(() => handleResendEmail(), 500);
const throttledLogin = throttle(() => handleLogin(), 5000);
const throttledRegister = throttle(() => handleRegister(), 5000);

const titles = ["Let’s Begin", "Welcome UIBricker!", "Join Us", "Register", "Sign Up", "We’re happy to have you!", "Become a UIBricker", "Create Your Account"];
const randomTitle = ref("");
onMounted(() => { randomTitle.value = titles[Math.floor(Math.random() * titles.length)]; });

function showNotification(type: 'success'|'error'|'warning'|'info'|'default', title: string, message: string, showResendBtn = false) {
  toast.add({
    title,
    description: message,
    color: type === 'success' ? 'success' : type === 'error' ? 'error' : 'info',
    duration: showResendBtn ? 0 : 5000,
    actions: showResendBtn ? [{ 
      label: cooldown.value > 0 ? `(${cooldown.value}s) ${t('actions.resend_email')}` : t('actions.resend_email'), 
      variant: 'solid', 
      color: 'primary', 
      onClick: debouncedResendEmail 
    }] : undefined
  });
}

async function handleRegister() {
  if (loading.value) return;
  loading.value = true;
  try {
    await validateWithRules(RegisterRules, form.value).catch((errs: string[]) => { throw { code: -1, msg: errs[0] }; });

    const registerResponse: { status: string } = await invoke('auth_register', { email: form.value.email, password: form.value.password });
    if (registerResponse.status === 'success') {
      showNotification('success', t('notifications.registered_title'), t('notifications.registered_message', { email: form.value.email }), true);
      startCooldown(60);
      activeTab.value = 'signin';
      return;
    }
    throw registerResponse;
  } catch (err: any) {
    showNotification('error', err.code != -1 ? t('notifications.error_with_code', { code: err.code }) : t('errors.invalid_input'), err.msg || t('errors.unexpected'), );
    console.error(err);
  } finally {
    loading.value = false;
  }
}

async function handleLogin() {
  if (loading.value) return;
  loading.value = true;
  try {
    await validateWithRules(LoginRules, form.value).catch((errs: string[]) => { throw { code: -1, msg: errs[0] }; });

    const loginResponse: { status: string } = await invoke('auth_login', { email: form.value.email, password: form.value.password });
    if (loginResponse.status === 'success') {
      router.push('/user');
      return;
    }
    throw loginResponse;
  } catch (err: any) {
    showNotification('error', err.code ? t('notifications.login_error_with_code', { code: err.code }) : t('errors.invalid_input'), err.msg || t('errors.unexpected'), err.error_code == 'email_not_confirmed');
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
    const resendResponse: { status: string, code: number, cooldown?: number } = await invoke('auth_resend_email', { email: form.value.email });
    if (resendResponse.status === 'success') {
      showNotification('success', t('notifications.email_sent_title'), t('notifications.registered_message', { email: form.value.email }), true);
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
