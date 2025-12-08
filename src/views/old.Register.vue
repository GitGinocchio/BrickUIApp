<template>
  <div class="container">
    <Header :sections="sections" />
    <n-card class="card-container">
      <div class="reg-slogan">
        <h1 class="title">{{ randomTitle }}</h1>
        <div class="divider"></div>
        <cite class="subtitle">
          lovely to see you here! 
          Register and be part of our family
        </cite>
      </div>
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
              <button class="toggle-btn" @click="showPassword = !showPassword">
                <EyesOpened v-if="showPassword" />
                <EyesClosed v-else />
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
                class="toggle-btn"
                @click="showConfirmPassword = !showConfirmPassword"
              >
                <EyesOpened v-if="showConfirmPassword" />
                <EyesClosed v-else />
              </button>
            </template>
          </NInput>
        </NFormItem>

        <NButton type="primary" block strong size="large" @click="handleSubmit">
          Register
        </NButton>
      </NForm>
    </n-card>
  </div>
</template>


<script setup lang="ts">
import { inject, Ref, ref, computed, onMounted } from 'vue'
import { NForm, NFormItem, NInput, NButton, FormInst, FormRules, FormItemRule, NCard } from 'naive-ui'
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

const sections = computed(() => {
  return [
    { icon: UserIcon, label: t('User'), onclick: () => router.push('/login-methods') },
    { icon: IdCard, label: t('Register')},
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

</style>