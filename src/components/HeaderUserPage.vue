<template>
    <div class="container-info">
            <n-image
                class="image-container"
                src="https://via.placeholder.com/150"
                alt="User Photo"
                width="150"
                height="150"
                >
                <input type="file"> 
                    <span>Change Photo</span>
                </input>
            </n-image>
    </div>
</template>

<script setup lang="ts">
import { NImage } from 'naive-ui';

</script>

<style scoped>

.container-info {
    padding: 50px;
    width: 100%;
    z-index: 1;
    height: 200px;
    display: flex;
    align-items: center;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.image-container {
    position: relative;
    width: 150px;
    height: 150px;
    border-radius: 50%;
    overflow: hidden;
    cursor: pointer;
}

.image-container span {
    position: relative;
    transform: translate(-50%, -50%);
    background: rgba(0, 0, 0, 0.5);
    color: #fff;
    text-align: center;
    padding: 5px 0;
    font-size: 14px;
    opacity: 0;
    transition: opacity 0.3s;
}


</style>