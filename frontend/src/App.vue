<script>
export default {
  name: 'App',
  data() {
    return {
      title: 'Page d\'accueil',
      csrfToken: window.CSRF_TOKEN
    }
  },
  methods: {
    testCsrf() {
      console.log('Utilisation du token CSRF:', this.csrfToken);
    }
  }
}
</script>

<template>
  <div class="home">
    <h1>{{ CryptoVoice }}</h1>
    <p>Welcome to CryptoVoice</p>
    <button @click="testCsrf">Tester le token CSRF</button>
  </div>
</template>

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