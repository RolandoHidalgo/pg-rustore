<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {Progress} from "@/components/ui/progress";
import {onMounted, ref} from "vue";

const r = useRouter()
const rr = useRoute()
//console.log(r,'')
console.log(rr.fullPath, 'asdasdasd')
const progress = ref(13)
onMounted(() => {
  const timer = setTimeout(() => {
    progress.value = 66
  }, 5000)
  return () => clearTimeout(timer)
})
</script>

<template>
  <div class="h-screen bg-purple-400 text-white text-3xl p-4 flex flex-col justify-center items-center gap-2">
    <div>Hola</div>
    <Progress :model-value="progress" class="w-[60%] text-white"/>

  </div>
</template>

<style scoped>

</style>