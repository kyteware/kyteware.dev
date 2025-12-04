window.getGumballs = () => {
  console.log("hi");

  return {
    "personal_projects": [
      0, 1, 2, 3, 4
    ],
    "experiences": [
      5, 6
    ],
    "events": [],
    "tidbits": [
      7, 8, 9
    ]
  }
}

window.shouldDrop = () => {
  return true;
}

window.doneDropping = (num) => {
  console.log("dropped: " + num);
}
