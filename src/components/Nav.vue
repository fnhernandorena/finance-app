<template>
    <div class="flex justify-around w-full text-xl">
      <a :href="'#/'" :class="{ 'border-b-4 border-sky-600 duration-300 font-bold': currentPath === '/'| ''}">Home</a> 
      <a :href="'#/money'" :class="{ 'border-b-4 border-sky-600 duration-300 font-bold': currentPath === '/money' |''}">Money</a> 
      <a :href="'#/capital'" :class="{ 'border-b-4 border-sky-600 duration-300 font-bold': currentPath === '/capital'| ''}">Capital</a>
    </div>
    <div>
      <component :is="currentView"/>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed } from 'vue'
  import Home from '../pages/Home.vue'
  import Money from '../pages/MoneyPage.vue'
  import Capital from '../pages/CapitalPage.vue'
  
  interface RouteMap {
    [key: string]: typeof Home | typeof Money | typeof Capital
  }
  
  const routes: RouteMap = {
    '/': Home,
    '/money': Money,
    '/capital': Capital
  }
  
  const currentPath = ref(window.location.hash.slice(1) || '/')
  
  window.addEventListener('hashchange', () => {
    currentPath.value = window.location.hash.slice(1) || '/'
  })
  
  const currentView = computed(() => {
    return routes[currentPath.value] || NotFound
  })
  </script>
  