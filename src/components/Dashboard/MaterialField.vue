<template>
  <div class="MaterialField">
    <input
      class="field"
      @input="$emit('update:modelValue', $event.target.value)"
      :value="modelValue"
      placeholder=" "
      type="text"
    />
    <label for="">Name of Listener</label>
    <span>{{ charLength }}</span>
    <div class="focus-bar" />
  </div>
</template>

<script>
export default {
  props: {
    modelValue: String,
  },
  data: () => ({
    text: "",
  }),
  computed: {
    charLength() {
      return this.modelValue.length;
    },
  },
};
</script>

<style lang="scss" scoped>
.MaterialField {
  width: 90%;
  position: relative;
  padding: 0px 10px;
  box-sizing: border-box;
  // margin-bottom: 40px;
  margin-left: 10px;
  margin-top: 20px;

  /* background: red; */
  label {
    position: absolute;
    bottom: 7px;
    left: 12px;
    color: getTextColor(dim);
    transition: 0.3s;
    font-size: 14px;
  }

  input {
    width: 100%;
    box-sizing: border-box;
    padding-top: 15px;
    height: 40px;
    text-indent: 5px;
    font-family: Comfortaa;
    background: transparent;
    font-size: 13px;
    color: white;
    outline: none;
    border: none;
    border-bottom: 1px solid #a4a4a4;
  }

  input:focus ~ label,
  .field:not(:placeholder-shown) + label {
    font-size: 9.5px;
    color: $success;
    bottom: 70%;
  }

  input:focus ~ .focus-bar {
    background: $success;
    width: calc(100% - 20px);
  }
  span {
    position: absolute;
    padding: 6px 6px 3px 6px;
    display: grid;
    place-items: center;
    right: 10px;
    bottom: 7px;
    border-radius: 4px;
    font-size: 9px;
    box-sizing: border-box;
    background: #9e9e9e;
    color: $success;
  }

  .focus-bar {
    transition: 0.3s;
    position: absolute;
    left: 10px;
    box-sizing: border-box;
    height: 1px;
    bottom: 0px;
    width: 0px;
  }

  .focus-bar.active {
    background: $success;
    width: calc(100% - 20px);
  }
}
</style>