<template>
  <div class="container">
    <h1>404 - Page Not Found</h1>
    <p>We couldn't find the page you're looking for.</p>
    <p>Please check the URL or navigate back to the <router-link to="/">home page</router-link>.</p>
  </div>
</template>

<style scoped>
.container {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
  text-align: center;
}

h1 {
  font-size: 3rem;
  color: #ff6347; /* Coral color */
}

p {
  font-size: 1.2rem;
  color: #333;
  margin-bottom: 20px;
}

.router-link-exact-active {
  color: #007bff; /* Blue color */
}
</style>
