<template>
  <div>
    <p v-if="amount">Income {{ amount }}</p>

    <form id="conversiion-form">
      <input type="number" v-model="form.amount" />
    </form>
    <button @click="setAmount()">convert</button>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";

@Component
export default class IncomeTab extends Vue {
  amount?: number;

  form: {
    currency: string;
    amount: number;
    date: string;
  } = {
    currency: "USD",
    amount: 0,
    date: "2022-03-11",
  };

  public setAmount(amount: number): number {
    return (this.amount = amount);
  }
}
</script>

<style scoped></style>
