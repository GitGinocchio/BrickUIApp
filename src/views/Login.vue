<template>
  <div class="container">
    <Header :sections="sections" />
    <n-card class="login-container">
      <div class="log-slogan">
        <h1 class="title">{{ randomTitle }}</h1>
        <div class="divider"></div>
        <cite class="subtitle">
          Lovely to see you again!  
          Log in to access your favorite bricks.
        </cite>
      </div>
      

      <!-- FORM -->
      <NForm
        :model="form"
        :rules="rules"
        ref="formRef"
        label-placement="top"
        require-mark-placement="right-hanging"
      >
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
                  <EyesOpened />
                </span>
                <span v-else>
                  <EyesClosed />
                </span>
              </button>
            </template>
          </NInput>
        </NFormItem>

        <!-- LOGIN BUTTON -->
        <NButton
          type="primary"
          block
          strong
          size="large"
          class="login-btn"
          @click="handleSubmit"
        >
          Login
        </NButton>
      </NForm>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import {
  NForm,
  NFormItem,
  NInput,
  NButton,
  FormInst,
  FormRules,
  NCard
} from 'naive-ui'
import EyesClosed from '../components/icons/EyesClosed.vue'
import EyesOpened from '../components/icons/EyesOpened.vue'
import Header from '../components/Header.vue'
import { User, AtSign } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const { t } = useI18n()
const router = useRouter()

const sections = computed(() => [
  { icon: User, label: t('User'), onclick: () => router.push('/login-methods') },
  { icon: AtSign, label: t('Login') }
])

const titles = [
  'Welcome Back, UIBricker!',
  'Back for more?',
  'Ready to Continue?',
  'Hey, good to see you again!',
  'You’re Back!',
  'Welcome Back',
  'Log In',
  'Access Your Account'
]

const randomTitle = ref('')

onMounted(() => {
  randomTitle.value = titles[Math.floor(Math.random() * titles.length)]
})

const form = ref({
  email: '',
  password: ''
})

const formRef = ref<FormInst | null>(null)
const showPassword = ref(false)

const rules: FormRules = {
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

const handleSubmit = async () => {
  try {
    await formRef.value?.validate()
    console.log('Valid form:', form.value)
  } catch (err) {
    console.warn('Invalid form:', err)
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

.log-slogan{
  color: white;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.title{
  font-size: 40px;
  margin: 0 0 0.5rem 0;
}

.divider {
  height: 0.1rem;
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
