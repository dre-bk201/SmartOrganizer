<template>
  <div id="RuleScreen">
    <Option>
      <template v-slot:header> Search Type </template>
      <template v-slot:content>
        <div class="radio-option">
          <input
            id="Filename"
            name="search"
            @click="$emit('update:type', $event.target.value)"
            type="radio"
            value="FileName"
          />
          <label for="Filename">Filename</label>
        </div>

        <div class="radio-option">
          <input
            id="FileExtension"
            name="search"
            type="radio"
            @input="$emit('update:type', $event.target.value)"
            value="FileExtension"
          />
          <label for="FileExtension">FileExtension</label>
        </div>
      </template>
    </Option>

    <Option title="Condition">
      <template v-slot:header> Condition </template>
      <template v-slot:content>
        <div class="radio-option">
          <input
            id="Includes"
            name="condition"
            @input="$emit('update:rule', $event.target.value)"
            type="radio"
            value="Includes"
          />
          <label for="Includes">Includes</label>
        </div>

        <div class="radio-option">
          <input
            id="Not Includes"
            name="condition"
            type="radio"
            value="Not Includes"
            @input="$emit('update:rule', $event.target.value)"
          />
          <label for="Not Includes">Not Includes</label>
        </div>
      </template>
    </Option>

    <input
      class="search"
      :value="modelValue"
      @input="$emit('update:modelValue', $event.target.value)"
      type="text"
      placeholder="Text to filter for"
    />
  </div>
</template>

<script>
import anime from "animejs";
import Option from "./Option.vue";

export default {
  components: {
    Option,
  },
  props: {
    modelValue: String,
  },
  methods: {},
  mounted() {
    const radioOptions = this.$el.querySelectorAll(".radio-option input");
    const type = this.$store.getters.getModalData.type_;
    const rule = this.$store.getters.getModalData.rule;

    radioOptions.forEach((element) => {
      if (element.value == rule) element.checked = true;
      if (element.value == type) element.checked = true;
    });

    anime({
      targets: this.$el,
      translateX: ["400px", "0px"],
      duration: 200,
      easing: "easeInSine",
    });
  },
  activated() {
    anime({
      targets: this.$el,
      translateX: ["400px", "0px"],
      duration: 200,
      easing: "easeInSine",
    });
  },
};
</script>

<style lang="scss" scoped>
#RuleScreen {
  padding: 35px 15px 0px 15px;
  height: 100%;
  box-sizing: border-box;
  overflow-y: auto;
  // background: red;

  .search {
    width: 100%;
    padding: 5px 0px 5px 0px;
    height: 30px;
    color: getTextColor(dim);
    text-indent: 10px;
    font-family: "Comfortaa";
    border: none;
    border-radius: 5px;
    box-shadow: 1px 1px 1px 0.5px rgba(0, 0, 0, 0.4);
    outline: none;
    background: $darkprimary;
    margin-bottom: 10px;
  }

  .radio-option {
    input {
      margin-right: 10px;
      margin-bottom: 15px;
    }

    label {
      font-family: "Comfortaa";
      color: getTextColor(dim);
      font-size: 16px;
    }
  }
}
</style>