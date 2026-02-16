<template>
  <div class="container">
    <Header :sections="headerSections">
      <template #actions>
        <div class="actions">
          <div class="edit-actions">
            <NButton v-if="editMode" @click="onEditConfirm" circle tertiary size="medium">
              <SaveAllIcon :size="20" />
            </NButton>
            <NButton v-if="editMode" @click="onEditDiscard" circle tertiary size="medium">
              <Trash2Icon :size="20" />
            </NButton>
            <NButton v-if="!editMode" @click="editMode = true" circle tertiary size="medium">
              <!--<Pencil v-if="!editMode" :size="16" />-->
              <Pencil :size="20" />
            </NButton>
          </div>
          <NButton circle tertiary><Settings2 :size="20" /></NButton>
        </div>
      </template>
    </Header>

    <NCard>
      <div class="card-content">
        <div class="profile-picture">
          <NImage
            src="..."
            alt="User Photo"
            width="128"
            height="128"
            :show-toolbar="false"
            :preview-disabled="true"
          >
            <template #error>
              <div class="default-avatar">
                <NIcon :size="64">
                  <UserRound :stroke-width="1" />
                </NIcon>
              </div>
            </template>
          </NImage>
          <div v-if="editMode" class="button" text>
            <NFloatButton>
              <Upload v-if="editMode" :size="18" />
            </NFloatButton>
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
              <NInput
                size="medium"
                :default-value="editUser.display_name ?? user.display_name" 
                @update:value="(value) => editUser.display_name = value.trim()" 
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
              <NInput
                size="small"
                :default-value="editUser.username ?? user?.username" 
                @update:value="(value) => editUser.username = value.trim()" 
              />
            </template>
          </EditableField>
        </div>
      </div>
    </NCard>

    <NCard>
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
    </NCard>

    <NCard>
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
              <NInput
                size="medium"
                type="textarea"
                :default-value="editUser.bio ?? user?.bio"
                @update:value="(value) => editUser.bio = value.trim()" 
              />
            </template>
          </EditableField>
        </div>
      </div>
    </NCard>

  </div>
</template>

<script setup lang="ts">
import { UserRound, Pencil, Settings2, Upload, UserIcon, SaveAllIcon, Trash2Icon } from 'lucide-vue-next';
import Header from '../components/Header.vue';
import { NCard, NIcon, NInput, NImage, NButton, NFloatButton, useNotification } from 'naive-ui';
import { computed, h, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { User } from 'interfaces/user';
import EditableField from '../components/EditableField.vue';

const headerSections = [
  { defaultIcon: () => h(UserIcon), label: 'User' }
]

const notify = useNotification();

const loading = ref<boolean>(true);
const user = ref<User>();

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

function hasChangesForField(key: keyof User) {
  const newVal = editUser.value[key]
  const oldVal = user.value[key]

  const normalize = (v: any) => {
    if (v === null || v === undefined) return ''
    if (typeof v === 'string') return v.trim()
    return v
  }

  return normalize(newVal) !== normalize(oldVal)
}

async function onEditConfirm() {
  const editedKeys = Object.keys(editUser.value) as (keyof User)[]

  const hasChanges = editedKeys.some(key => hasChangesForField(key))

  if (hasChanges) {
    console.log('sending update', editUser.value)
    const updated: User = await invoke("users_update_me", { update: editUser.value })
    user.value = updated
  }

  editUser.value = {}
  editMode.value = false
}


async function onEditDiscard() {
  editUser.value = {};
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