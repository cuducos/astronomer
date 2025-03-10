import init, { color } from "./frontend.js";

const configFor = function (name, stars, data) {
  return {
    type: "bar",
    data: data,
    options: {
      indexAxis: "y",
      responsive: true,
      plugins: {
        legend: { display: false },
        title: {
          display: true,
          text: `${name}'s GitHub ⭐ ${stars.toLocaleString()} Stargazers`,
          font: { size: 20, weight: "bold" },
        },
        tooltip: {
          font: { size: 14 },
          callbacks: {
            title: (context) =>
              `${context[0].dataset.language} ⭐ ${context[0].dataset.total.toLocaleString()}`,
          },
        },
      },
      scales: {
        x: { stacked: true },
        y: { stacked: true, ticks: { font: { size: 16 } } },
      },
    },
  };
};

const dataFor = function (languages) {
  return {
    labels: languages.map((language) => language.name),
    datasets: languages
      .map((language, idx) => {
        return language.source.map((source, i) => {
          let data = Array(languages.length).fill(0);
          data[idx] = source.stars;
          return {
            label: source.repository,
            data: data,
            backgroundColor: color(language.color, i, language.source.length),
            borderRadius: 2,
            language: language.name,
            total: language.stars,
          };
        });
      })
      .flat(),
  };
};

const run = function () {
  const chart = document.getElementById("chart");
  const label = document.getElementById("desc");
  label.hidden = true;
  fetch(`/${chart.dataset.userName}.json${window.location.search}`).then(
    (response) => {
      if (!response.ok) {
        label.innerText = `Error loading data from GitHub: is ${chart.dataset.userName} the right username?`;
        label.hidden = false;
        return;
      }
      response.json().then((astronomer) => {
        new Chart(
          chart,
          configFor(
            astronomer.name,
            astronomer.stars,
            dataFor(astronomer.languages),
          ),
        );
        label.hidden = false;
      });
    },
  );
};

window.onload = () => {
  init().then(() => {
    if (document.readyState !== "loading") return run();
    document.addEventListener("DOMContentLoaded", run);
  });
};
