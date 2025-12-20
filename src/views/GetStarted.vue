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
            @keydown.enter.prevent="handleRegister"
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

            <NButton class="form-button" type="primary" :loading="loading" block strong size="large" @click="handleRegister">
              Register
            </NButton>
          </n-form>
        </n-tab-pane>
        <n-tab-pane name="signin" tab="Sign In">
          <n-form
            :model="form"
            :rules="LoginRules"
            ref="signinFormRef"
            @keydown.enter.prevent="handleLogin"
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
                  <button class="toggle-btn" type="button" @click.left.stop="showSigninPassword = !showSigninPassword" @submit="() => {}">
                    <EyesOpened v-if="showSigninPassword" />
                    <EyesClosed v-else />
                  </button>
                </template>
              </n-input>
            </n-form-item-row>
            <NButton class="form-button" type="primary" :loading="loading" block strong size="large" @click="handleLogin">
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
  </n-alert>
    </div>
  </div>
</template>


<script setup lang="ts">
import { inject, Ref, ref, computed, onMounted } from 'vue'
import { NForm, NFormItem, NInput, NButton, FormInst, FormRules, FormItemRule, NCard, NTabs, NTabPane, NFormItemRow, NAlert } from 'naive-ui'
import EyesClosed from '../components/icons/EyesClosed.vue'
import EyesOpened from '../components/icons/EyesOpened.vue'
import { fetch } from '@tauri-apps/plugin-http';
import { useI18n } from "vue-i18n";
import { UserIcon } from 'lucide-vue-next';
import { User } from 'interfaces/user';
import { useRouter } from "vue-router";
import Header from '../components/Header.vue';

const { t } = useI18n();
const router = useRouter();
const titles = ["Let’s Begin", "Welcome UIBricker!", 
"Join Us", "Register", "Sign Up", "We’re happy to have you!", "Become a UIBricker", "Create Your Account"];

const randomTitle = ref("");

onMounted(() => {
  randomTitle.value = titles[Math.floor(Math.random() * titles.length)];
});

const sections = computed(() => {
  return [
    { icon: UserIcon, label: t('User')}
  ]
});

const form = ref({
  email: '',
  password: '',
  confirmPassword: ''
})

const signupformRef = ref<FormInst | null>(null);
const signinFormRef = ref<FormInst | null>(null);
const showSigninPassword = ref(false);
const showSignupPassword = ref(false);
const showSignupConfirmPassword = ref(false);
const activeTab = ref('signup');
const user = inject("user") as Ref<User|null>;

const loading = ref<boolean>(false);

const showAlert = ref(false);
const alertType = ref<"warning" | "error" | "success" | "default" | "info">('error');
const alertTitle = ref<string>('');
const alertMessage = ref<string>('');


const validatePasswordMatch = (rule: FormItemRule, value: string): boolean | Error => {
  if (value !== form.value.password) {
    return new Error('Passwords do not match')
  }
  return true
}

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

async function handleRegister() {
  try {
    loading.value = true;
    await signupformRef.value?.validate().catch((warnings) => {
      throw { code: -1, msg: warnings[0][0].message };
    });
    const payload = {
      email : form.value.email,
      password : form.value.password,
    };
    
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));
    const response = await fetch("https://brickui.app/api/auth/register/classic", {
      method: 'POST',
      body: body
    });

    let registerResponse: { msg: string, code: number } = { 
      msg: 'Something went wrong when sending the request', 
      code: null 
    };
    try {
      const text = await response.text();
      registerResponse = JSON.parse(text);
    }
    catch (e) {
      throw { 
        code: registerResponse.code ?? response.status, 
        message: registerResponse.msg ?? response.statusText
      };
    }

    if (registerResponse.code === 200 || (response as any).ok) {
      // success
      alertTitle.value = 'Successfully registered!';
      alertMessage.value = `We've sent a confirmation email to ${payload.email}.\nClick the link to activate your account.`
      alertType.value = 'success'
      showAlert.value = true;
      activeTab.value = 'signin'
      return;
    }

    throw registerResponse;
  }
  catch (err: any) {
    alertType.value = 'error';
    showAlert.value = true;

    console.error(err);

    switch (err.code) {
      case 500:
        alertTitle.value = 'Internal Server Error';
        break;
      case 400:
        alertTitle.value = 'Validation Failed';
        break;
      case 422:
        alertTitle.value = 'Email Exists';
        break;
      case 409:
        alertTitle.value = 'Conflict';
        break;
      case -1:
        alertTitle.value = 'Client Error';
        alertMessage.value = err.msg
        break;
      default:
        alertTitle.value = `Login Error: ${err.code}` || 'Unhandled Error';
        alertMessage.value = err.msg
        break;
    }
  }
  finally {
    loading.value = false;
  }
}

async function handleLogin() {
  try {
    loading.value = true;
    await signinFormRef.value?.validate().catch((warnings) => {
      throw { code: -1, msg: warnings[0][0].message };
    });

    let payload = {
      email : form.value.email,
      password : form.value.password
    }

    // Trasformiamo tramite un encoder il payload in un array di variabili:
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));

    const response = await fetch("https://brickui.app/api/auth/login", {
      method: 'POST',
      body: body
    });

    let loginResponse: { msg: string, code: number } = { 
      msg: 'Something went wrong when sending the request', 
      code: null 
    };
    try {
      const text = await response.text();
      loginResponse = JSON.parse(text);
    }
    catch (e) {
      throw { 
        code: loginResponse.code ?? response.status, 
        message: loginResponse.msg ?? response.statusText
      };
    }

    if(loginResponse.code === 200 || (response as any).ok){
      // Ricordarsi di fare la logica del token
      router.push('/user');
      return;
    }

    throw loginResponse
  }
  catch (err) {
    alertType.value = 'error';
    showAlert.value = true;

    console.error(err);

    switch (err.code) {
      case -1:
        alertTitle.value = 'Client Error';
        alertMessage.value = err.msg;
        break;
      case 400:
        alertTitle.value = err.msg;
        alertMessage.value = 'Click the link we sent to your inbox to activate your account.'
        break;
      default:
        alertTitle.value = `Login Error: ${err.code}` || 'Unhandled Error';
        alertMessage.value = err.msg;
        break;
    }
  }
  finally {
    loading.value = false;
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

.form-button {
  margin-top: 1rem;
}
</style>