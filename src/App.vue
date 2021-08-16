<template>
  <div id="app">
    <div id="nav">
      <div class="container">
        <div class="bar" />

        <Navlink route="/" distance="0px" name="Dashboard" />
        <Navlink route="/logs" distance="105px" name="Logs" />
        <Navlink route="/statistics" distance="210px" name="Statistics" />

        <div class="align-bottom">
          <BroomAnimation v-if="$store.getters.isCleaning" />
        </div>

        <router-view v-slot="{ Component }">
          <keep-alive include="Dashboard">
            <component :is="Component" distance="0px" name="Dashboard" />
          </keep-alive>
        </router-view>
      </div>
    </div>
  </div>
</template>
<script>
import Navlink from "@/components/Navigation/Navlink";
import BroomAnimation from "./components/Global/BroomAnimation.vue";
export default {
  components: {
    Navlink,
    BroomAnimation,
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
  background: $darkprimary;
  height: 100vh;
  width: 100vw;
  position: relative;

  #nav {
    height: 100vh;
    width: $nav-width;
    background: $darkprimary;
    font-size: 0.8rem;
    flex-grow: 1;

    .container {
      @include flexAlignCenter(column);
      padding-top: 50px;
      .align-bottom {
        position: absolute;
        bottom: 10px;
      }
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
