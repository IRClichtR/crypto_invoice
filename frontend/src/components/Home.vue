<template>
    <div class="home">
      <h1>{{ CryptoVoice }}</h1>
      <p>Hello CryptoVoice</p>
      <button @click="fetchData">This is a button</button>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  interface ApiResponse {
    success: boolean;
    data?: any;
    message?: string;
  }
  
  export default defineComponent({
    name: 'HomeView',
    data() {
      return {
        title: 'Home Page',
        csrfToken: window.CSRF_TOKEN as string
      }
    },
    methods: {
      async fetchData(): Promise<void> {
        try {
          const response = await fetch('/api/data', {
            method: 'GET',
            headers: {
              'Content-Type': 'application/json',
              'X-CSRF-Token': this.csrfToken
            }
          });
          
          const result: ApiResponse = await response.json();
          
          if (result.success) {
            console.log('Data received', result.data);
          } else {
            console.error('Error:', result.message);
          }
        } catch (error) {
          console.error('Request error:', error);
        }
      }
    }
  });
  </script>
  
  <style scoped>
  .home {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    border-radius: 8px;
  }
  
  h1 {
    color: #2c3e50;
  }
  
  button {
    background-color: #4CAF50;
    border: none;
    color: white;
    padding: 10px 20px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin: 4px 2px;
    cursor: pointer;
    border-radius: 4px;
  }
  </style>
  