import dataLabels from "chartjs-plugin-datalabels";

const MONTHS = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];

const defaultObject = (obj, key, value) => {
  if (obj[key] === undefined) obj[key] = value;
  else {
    obj[key] = obj[key] + value;
  }
};

const generateChartData = function (chartType, listeners) {
  if (chartType == "line") {
    let dataset = {};
    let positions = [];

    for (let i = 0; i < listeners.length; i++) {
      listeners[i].logs.forEach((log) => {
        const date = new Date(log.timestamp);
        defaultObject(dataset, MONTHS[date.getMonth()], 1);
        positions.push(date.getMonth());
      });
    }

    // listeners.forEach((item) => {
    //   item.logs.forEach((log) => {
    //     const date = new Date(log.timestamp);
    //     defaultObject(dataset, MONTHS[date.getMonth()], 1);
    //     positions.push(date.getMonth());
    //   });
    // });

    let upperBoundMonth = positions.reduce((a, b) => {
      return Math.max(a, b);
    });

    let data = [];

    MONTHS.forEach((month) => {
      if (Object.keys(dataset).indexOf(month) < 0) data.push(0);
      else {
        data.push(dataset[month]);
      }
    });

    console.log(data);

    return {
      labels: MONTHS.slice(0, upperBoundMonth + 1),
      data,
    };
  } else if (chartType == "doughnut") {
    let labels = [];
    let data = [];

    for (let i = 0; i < listeners.length; i++) {
      labels.push(listeners[i].title);
      data.push(listeners[i].logs.length);
    }
    return {
      labels,
      data,
    };
  }
};

export const generateChart = (chartType, listeners) => {
  const { labels, data } = generateChartData(chartType, listeners);
  if (chartType == "line") {
    return {
      type: chartType,
      plugins: [dataLabels],

      data: {
        labels,
        datasets: [
          {
            label: "Monthly Activity",
            data,
            fill: false,
            borderColor: "#41B883",
            tension: 0.5,
            backgroundColor: "#555",
          },
        ],
      },
      options: {
        fill: false,
        maintainAspectRatio: false,
        interaction: {
          intersect: false,
        },
        radius: 0,
        plugins: {
          datalabels: {
            backgroundColor: function (context) {
              return context.dataset.backgroundColor;
            },
            borderRadius: 4,
            color: "white",
            font: {
              weight: "bold",
            },
            formatter: Math.round,
            padding: 6,
          },
        },
      },
    };
  } else if (chartType == "doughnut") {
    return {
      type: chartType,
      options: { responsive: true, maintainAspectRatio: false },
      data: {
        labels,
        datasets: [
          {
            backgroundColor: ["#41B883", "#E46651", "#00D8FF", "#DD1B16"],
            data,
          },
        ],
      },
    };
  }
};
