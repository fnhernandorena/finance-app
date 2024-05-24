<template>
    <div>
      <h1>Vehicles</h1>
      <ul>
        <li v-for="vehicle in vehicles" :key="vehicle.id">
          {{ vehicle.brand }} {{ vehicle.model }} ({{ vehicle.year }}) - ${{ vehicle.value }}
          <button @click="deleteVehicle(vehicle.id)">Delete</button>
          <button @click="editVehicle(vehicle.id)">Edit</button>
        </li>
      </ul>
      <form @submit.prevent="addVehicle">
        <input v-model="newVehicle.brand" placeholder="Brand" />
        <input v-model="newVehicle.model" placeholder="Model" />
        <input v-model.number="newVehicle.year" placeholder="Year" />
        <input v-model.number="newVehicle.value" placeholder="Value" />
        <button type="submit">Add Vehicle</button>
      </form>
    </div>
  </template>
  
  <script setup>
  import { ref, onMounted } from 'vue';
  import { vehicles, loadVehicles, createVehicle, deleteVehicle, updateVehicle } from '../services/vehiclesService.ts';
  
  const newVehicle = ref({ brand: '', model: '', year: 0, value: 0 });
  
  const addVehicle = () => {
    createVehicle(newVehicle.value.brand, newVehicle.value.model, newVehicle.value.year, newVehicle.value.value);
    newVehicle.value = { brand: '', model: '', year: 0, value: 0 };
  };
  
  const editVehicle = (id) => {
    const vehicle = vehicles.value.find(v => v.id === id);
    if (vehicle) {
      const updatedVehicle = { ...vehicle, brand: 'Updated Brand', model: 'Updated Model', year: vehicle.year + 1, value: vehicle.value + 1000 };
      updateVehicle(updatedVehicle.id, updatedVehicle.brand, updatedVehicle.model, updatedVehicle.year, updatedVehicle.value);
    }
  };
  
  onMounted(() => {
    loadVehicles();
  });
  </script>
  