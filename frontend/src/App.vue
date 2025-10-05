<template lang="pug">
h1 Hello App!
nav
  ul
    li
      RouterLink(to="/") Go to Home
    li
      RouterLink(to="/texture") Go to Texture
main
  RouterView 
</template>
