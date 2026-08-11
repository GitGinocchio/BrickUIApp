<template>
  <div class="container">
    <Header :sections="headerSections">
      <template #actions>
        <div class="actions">
          <div class="edit-actions">
            <UButton v-if="editMode" @click="onEditConfirm" variant="ghost" class="px-2 py-1">
              <SaveAllIcon :size="20" />
            </UButton>
            <UButton v-if="editMode" @click="onEditDiscard" variant="ghost" class="px-2 py-1">
              <Trash2Icon :size="20" />
            </UButton>
            <UButton v-if="!editMode" @click="editMode = true" variant="ghost" class="px-2 py-1">
              <Pencil :size="20" />
            </UButton>
          </div>
          <UButton variant="ghost"><Settings2 :size="20" /></UButton>
        </div>
      </template>
    </Header>

    <div class="card-content">
      <div>
        <div class="profile-picture">
          <img src="..." alt="User Photo" width="128" height="128" class="rounded-full object-cover" />
          <div v-if="editMode" class="button" text>
            <UButton variant="ghost" size="sm">
              <Upload v-if="editMode" :size="18" />
            </UButton>
          </div>
        </div>

        <div class="user-info">
          <EditableField 
            :loading="loading" 
            :canEdit="editMode" 
            width="30vw"
            size="medium"
            @discard="() => delete editUser.display_name"
          >
            <template #view>
              <h2>{{ displayName }}</h2>
            </template>
            <template #edit>
              <UInput
                model-value="editUser.display_name"
                :default-value="editUser.display_name ?? user.display_name" 
                @update:model-value="(value) => editUser.display_name = value.trim()" 
              />
            </template>
          </EditableField>
          <EditableField
            :loading="loading" 
            :canEdit="editMode" 
            width="30vw"
            size="small"
            @discard="() => delete editUser.username"
          >
            <template #view>
              <p>@{{ editUser.username ?? user?.username }}</p>
            </template>
            <template #edit>
              <UInput
                model-value="editUser.username"
                :default-value="editUser.username ?? user?.username" 
                @update:model-value="(value) => editUser.username = value.trim()" 
              />
            </template>
          </EditableField>
        </div>
      </div>
    </div>

    <div class="card-content">
      <div class="user-info">
        <h2>Email</h2>
        <EditableField
          :loading="loading"
          :canEdit="editMode"
          width="30vw"
          size="medium"
        >
          <template #view>
            <h4>{{ user.email }}</h4>
          </template>
        </EditableField>
      </div>
    </div>

    <div class="card-content">
      <div class="user-info">
        <h2>Bio</h2>
        <EditableField 
          :loading="loading" 
          :canEdit="editMode" 
          width="50vw"
          height="10vw"
          size="medium"
          @discard="() => delete editUser.bio"
        >
          <template #view>
            <p>{{ displayBio }}</p>
          </template>
          <template #edit>
            <UInput
              model-value="editUser.bio"
              type="textarea"
              :default-value="editUser.bio ?? user?.bio"
              @update:model-value="(value) => editUser.bio = value.trim()" 
            />
          </template>
        </EditableField>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Pencil, Settings2, Upload, UserIcon, SaveAllIcon, Trash2Icon } from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';

const { t } = useI18n();

import type { User } from '#interfaces';
import Header from '#components/Header.vue';
import EditableField from '#components/EditableField.vue';

const headerSections = [
  { defaultIcon: () => h(UserIcon), label: t('user.title') }
]

const loading = ref<boolean>(true);
const user = ref<Partial<User>>();

const editMode = ref<boolean>(false);
const editUser = ref<Partial<User>>({});

const displayName = computed(() => {
  const editVal = editUser.value.display_name?.trim()
  const userVal = user.value.display_name?.trim()

  if (editVal == '' && userVal != '') {
    return user.value.username
  }
  else if (editVal && editVal != '') {
    return editVal
  } else if (userVal && userVal !== '') {
    return userVal
  } else {
    return user.value.username
  }
})

const displayBio = computed(() => {
  const editVal = editUser.value.bio?.trim()
  const userVal = user.value.bio?.trim()
  const defaultMsg = "You don't have a bio, describe yourself!"

  if (userVal && editVal === '') {
    return defaultMsg
  }
  else if (editVal && editVal !== '') {
    return editVal
  } else if (userVal && userVal !== '') {
    return userVal
  } else {
    return defaultMsg
  }
})

function hasChangesForField<K extends keyof User>(key: K): boolean {
  const newVal = (editUser.value as Record<string, unknown> | undefined)?.[key as string];
  const oldVal = (user.value as Record<string, unknown> | undefined)?.[key as string];

  const normalize = (v: unknown): string | number | boolean => {
    if (v === null || v === undefined) return '';
    if (typeof v === 'string') return v.trim();
    if (typeof v === 'object') return JSON.stringify(v);
    return v as string | number | boolean;
  };

  return normalize(newVal) !== normalize(oldVal);
}

async function onEditConfirm() {
  const editedKeys = Object.keys(editUser.value) as (keyof User)[]

  const hasChanges = editedKeys.some(key => hasChangesForField(key))

  if (hasChanges) {
    console.log('sending update', editUser.value)
    const updated: User = await invoke("users_update_me", { update: editUser.value })
    user.value = updated
  }

  editUser.value = {} as Partial<User>
  editMode.value = false
}


async function onEditDiscard() {
  editUser.value = {} as Partial<User>;
  editMode.value = false;
}

onMounted(async () => {
  try {
    user.value = await invoke("users_get_me");
    console.log(user.value);
    loading.value = false;
  } catch (error: any) {
    
  }
});

</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  padding: 1rem;
  gap: 1rem;
}

.actions { 
  display: flex;
  flex-direction: row;
  gap: 0.75rem;
}

.actions .edit-actions {
  display: flex;
  flex-direction: row;
  gap: 0.25rem;
}


.card-content {
  display: flex;
  flex-direction: row;
  gap: 1rem;
}

.profile-picture {
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
}

.profile-picture .button {
  display: flex;
  position: absolute;
  top: 70%;
  left: 70%;
  transform: translate(-25%, -25%);
  z-index: 10;
}

.default-avatar {
  display: flex;
  padding: 2rem;
  border: 0.2rem solid rgba(255,255,255, 0.82);
  border-radius: 50%;
}

.user-info {
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 0.25rem;
}

.user-info .field {
  display: flex;
  flex-direction: row;
  gap: 1rem;
}

.user-info h2 {
  margin: 0;
}

.user-info p {
  margin: 0;
}
</style>