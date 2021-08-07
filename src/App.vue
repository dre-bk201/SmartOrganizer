<template>
  <div id="app">
    <!-- <Navbar /> -->
    <div id="nav">
      <div class="container">
        <div class="bar" />

        <!-- <keep-alive include="Dashboard"> -->
        <Navlink route="/" distance="0px" name="Dashboard" />
        <!-- </keep-alive> -->
        <Navlink route="/logs" distance="105px" name="Logs" />
        <Navlink route="/statistics" distance="210px" name="Statistics" />

        <router-view v-slot="{ Component }">
          <keep-alive include="Dashboard">
            <component :is="Component" distance="0px" name="Dashboard" />
          </keep-alive>
        </router-view>
      </div>
    </div>
    <!-- <keep-alive include="Dashboard"> -->
    <!-- </keep-alive> -->
  </div>
</template>
<script>
import Navlink from "@/components/Navigation/Navlink";

export default {
  components: {
    Navlink,
  },
  methods: {
    output(str) {
      console.log("Output: ", str);
    },
  },
};
</script>

<style lang="scss">
* {
  margin: 0px;
  padding: 0px;
  text-decoration: none;
}

#app {
  // background: red;
  background: $darkprimary;
  height: 100vh;
  width: 100vw;
  position: relative;

  #nav {
    height: 100vh;
    width: $nav-width;
    background: $darkprimary;
    font-size: 0.8rem;

    .container {
      @include flexAlignCenter(column);
      padding-top: 50px;
    }

    .bar {
      background: $primary;
      height: 63px;
      width: 3px;
      position: absolute;
      left: 0px;
    }
  }
}
</style>
