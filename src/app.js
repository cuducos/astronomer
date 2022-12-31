const configFor = function (name, stars, data) {
  return {
    type: "bar",
    data: data,
    options: {
      indexAxis: "y",
      responsive: true,
      plugins: {
        legend: {
          position: "right",
        },
        title: {
          display: true,
          text: `${name}'s GitHub ${stars} Stats`,
        },
      },
    },
  };
};

const dataFor = function (languages) {
  return {
    labels: ["Stars"],
    datasets: languages.map((language) => {
      return {
        label: language.name,
        data: [language.stars],
        backgroundColor: language.color,
      };
    }),
  };
};

const init = function () {
  const label = document.getElementById("desc");
  label.hidden = true;

  const chart = document.getElementById("chart");
  fetch(`/${chart.dataset.userName}.json`).then((response) =>
    response.json().then((astronomer) => {
      new Chart(
        chart,
        configFor(
          astronomer.name,
          astronomer.stars,
          dataFor(astronomer.languages)
        )
      );
      label.hidden = false;
    })
  );
};

window.onload = () => {
  if (document.readyState !== "loading") {
    init();
  } else {
    document.addEventListener("DOMContentLoaded", init);
  }
};
