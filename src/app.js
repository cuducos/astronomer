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

const colorFor = function (hex, position, total) {
  if (total <= 1) return hex;
  const percent = 1 - position / (total - 1);
  const r = parseInt(hex.substring(1, 3), 16);
  const g = parseInt(hex.substring(3, 5), 16);
  const b = parseInt(hex.substring(5, 7), 16);
  const a = 0.25 + percent * 0.75;
  return `rgba(${r}, ${g}, ${b}, ${a})`;
};

const dataFor = function (languages) {
  return {
    labels: languages.map((language) => language.name),
    datasets: languages
      .map((language, idx) => {
        return language.source.map((source, i) => {
          let data = Array(languages.length).fill(0);
          data[idx] = source.stars;
          const color = colorFor(language.color, i, language.source.length);
          return {
            label: source.repository,
            data: data,
            backgroundColor: color,
            borderRadius: 2,
            language: language.name,
            total: language.stars,
          };
        });
      })
      .flat(),
  };
};

const relevant = function (languages) {
  if (languages.length <= 1) return languages;
  const threshold = languages[0].stars * 0.01;
  return languages.filter((language) => language.stars >= threshold);
};

const init = function () {
  const label = document.getElementById("desc");
  label.hidden = true;
  const chart = document.getElementById("chart");
  fetch(`/${chart.dataset.userName}.json${window.location.search}`).then(
    (response) =>
      response.json().then((astronomer) => {
        const languages = relevant(astronomer.languages);
        new Chart(
          chart,
          configFor(astronomer.name, astronomer.stars, dataFor(languages)),
        );
        label.hidden = false;
      }),
  );
};

window.onload = () => {
  if (document.readyState !== "loading") {
    init();
  } else {
    document.addEventListener("DOMContentLoaded", init);
  }
};
