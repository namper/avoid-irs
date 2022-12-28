import { useState } from "react";
import DatePicker from "react-datepicker";
import { getExchangedCurrency } from "./api/api";
import "react-datepicker/dist/react-datepicker.css";

function App() {
  const [startDate, setStartDate] = useState(new Date());

  return (
    <div>
      <h1> Pick date of income </h1>
      <DatePicker
        selected={startDate}
        onChange={(date: Date) => setStartDate(date)}
      />
      <button
        onClick={() => {
          getExchangedCurrency({
            date: startDate,
            currency: "usd",
            amount: 1000,
          })
            .then((res) => {
              console.log("data", res.data);
            })
            .catch(console.error);
        }}
      >
        {" "}
        Compute{" "}
      </button>
    </div>
  );
}

export default App;
