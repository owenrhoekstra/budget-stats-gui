<script setup lang="ts">
import { ElButton as Button } from 'element-plus'
import { ref } from 'vue'

const click = ref(0)

function handleClick() {
  console.log('click')
  click.value++
}
</script>

<template>
  <Button @click="handleClick">
    Click Me
  </Button>
  <p>{{click}}</p>
</template>

<style scoped>

</style>
