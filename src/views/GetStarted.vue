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
        @active-name-change="(newTab) => activeTab = newTab"
        default-value="signin"
        size="large"
        animated
      >
        <n-tab-pane name="signin" tab="Sign In">
          <n-form
          :model="form"
          :rules="LoginRules"
          ref="signinFormRef"
          >
            <n-form-item-row label="Username" path="username">
              <n-input v-model:value="form.username" placeholder="Username"/>
            </n-form-item-row>
            <n-form-item-row label="Password" path="password">
              <n-input
              v-model:value="form.password"
              :type="showSigninPassword ? 'text' : 'password'"
              placeholder="Password"
            >
              <template #suffix>
                <button class="toggle-btn" @click.stop="showSigninPassword = !showSigninPassword">
                  <EyesOpened v-if="showSigninPassword" />
                  <EyesClosed v-else />
                </button>
              </template>
            </n-input>

            </n-form-item-row>
            <NButton type="primary" block strong size="large" @click="handleLogin">
              Login
            </NButton>
          </n-form>
        </n-tab-pane>
        <n-tab-pane name="signup" tab="Sign Up">
          <n-form
          :model="form" 
          :rules="RegisterRules"
          ref="signupformRef">
            <n-form-item-row label="Username" path="username">
              <n-input v-model:value="form.username" placeholder="Username"/>
            </n-form-item-row>
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
                  <button class="toggle-btn" @click.stop="showSignupPassword = !showSignupPassword">
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
                  <button class="toggle-btn" @click.stop="showSignupConfirmPassword = !showSignupConfirmPassword">
                    <EyesOpened v-if="showSignupConfirmPassword" />
                    <EyesClosed v-else />
                  </button>
                </template>
              </n-input>
            </n-form-item-row>

            <NButton type="primary" block strong size="large" @click="handleRegister">
              Register
            </NButton>
          </n-form>
        </n-tab-pane>
      </n-tabs>
    </n-card>
    <div class="alert">
        <n-alert
    v-if="showAlert"
    type="error"
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
import { UserIcon, IdCard } from 'lucide-vue-next';
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


const form = ref({
  username: '',
  email: '',
  password: '',
  confirmPassword: ''
})

const signupformRef = ref<FormInst | null>(null);
const signinFormRef = ref<FormInst | null>(null);
const showSigninPassword = ref(false);
const showSignupPassword = ref(false);
const showSignupConfirmPassword = ref(false);
const activeTab = ref('signin');
const user = inject("user") as Ref<User|null>;

const showAlert = ref(false);
const alertTitle = ref<string>('')
const alertMessage = ref<string>('');


const validatePasswordMatch = (rule: FormItemRule, value: string): boolean | Error => {
  if (value !== form.value.password) {
    return new Error('Passwords do not match')
  }
  return true
}

const LoginRules: FormRules = {
  username: [
    { required: true, message: 'Username is required', trigger: ['blur', 'input'] },
    { min: 3, message: 'Username must be at least 3 characters', trigger: 'input' },
    { max: 20, message: 'Username must be less than 20 characters', trigger: 'input' }
  ],
  password: [
    { required: true, message: 'Password is required', trigger: ['blur', 'input'] },
    { min: 8, message: 'Password must be at least 8 characters', trigger: 'input' },
    { max: 128, message: 'Password too long', trigger: 'input' }
  ]
}

const RegisterRules: FormRules = {
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

async function handleRegister() {
  let payload;
  try {
    if (activeTab.value === 'signin'){
      await signinFormRef.value?.validate();
      payload = {
        username : form.value.username,
        password : form.value.password,
      };
    }
    else{
      await signupformRef.value?.validate()
      payload = {
        username : form.value.username,
        email : form.value.email,
        password : form.value.password
        /**
         * TODO : Ricordarsi di fare l'n-input per il telefono
         */
      };

    }
    console.log('Valid form:', form.value)
    
    // Trasformiamo tramite un encoder il payload in un array di variabili:
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));
    
    console.log(body);

    const response = await fetch("https://brickui.app/api/auth/register/classic", {
      method: 'POST',
      body: body
    });

    console.log('HTTP response status:', (response as any).status, (response as any).statusText);

    const text = await response.text();
    console.log('Server response text:', text);
    let registerResponse: any = {};
    try {
      registerResponse = JSON.parse(text);
    }
    catch (e) {
      // non-JSON response: show raw text and HTTP status
      alertTitle.value = 'Invalid Server Response';
      alertMessage.value = text || `HTTP ${(response as any).status}`;
      showAlert.value = true;
      return;
    }

    // normalize code and message using JSON payload or HTTP status as fallback
    const code = registerResponse.code ?? (response as any).status;
    const msg = registerResponse.msg ?? registerResponse.message ?? text ?? (response as any).statusText;
    alertMessage.value = msg;

    console.log('Parsed register response:', registerResponse, 'code fallback:', code);

    if (code === 200 || code === '200' || (response as any).ok) {
      // success
      router.push('/user');
      return;
    }

    // handle common error codes
    switch (Number(code)) {
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
      default:
        alertTitle.value = `Error ${code}` || 'Unhandled Error';
        console.log('Unhandled Error', registerResponse);
        break;
    }

    showAlert.value = true;
  }
  catch (err) {
    console.warn('Invalid form:', err)
  }
}

async function handleLogin(){
  let payload;
  try {
    payload = {
      username : form.value.username,
      password : form.value.password
    }

    // Trasformiamo tramite un encoder il payload in un array di variabili:
    const encoder = new TextEncoder();
    const body = encoder.encode(JSON.stringify(payload));

    const response = await fetch("https://brickui.app/api/auth/login", {
      method: 'POST',
      body: body
    });

    const text = await response.text();
    let loginResponse: any = {};
    try {
      loginResponse = JSON.parse(text);
    }
    catch (e) {
      // non-JSON response: show raw text and HTTP status
      alertTitle.value = 'Invalid Server Response';
      alertMessage.value = text || `HTTP ${(response as any).status}`;
      showAlert.value = true;
      return;
    }

    const code = loginResponse.code ?? (response as any).status;
    const msg = loginResponse.msg ?? loginResponse.message ?? text ?? (response as any).statusText;
    alertMessage.value = msg;

    if(code === 200 || code === '200' || (response as any).ok){
      // Ricordarsi di fare la logica del token
      router.push('/user');
      return;
    }
    switch (Number(code)){
      // Gestire casi errori
      default:
        alertTitle.value = `Login Error ${code}` || 'Unhandled Error';
        console.log('Login Error', loginResponse);
        break;
    }
    showAlert.value = true;
  }
  catch (err) {
    console.warn('Invalid form:', err)
    alertTitle.value = 'Client Error';
    alertMessage.value = String(err);
    showAlert.value = true;
  }
}

const sections = computed(() => {
  return [
    { icon: UserIcon, label: t('User')}
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

</style>