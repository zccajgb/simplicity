import { Task } from "@/models/task";

export function createRepeat(task) {
  if (!task.repeat) {
    return;
  }
  let newTask = Task.clone(task);

  newTask.date = task.date ?? new Date();
  if (task.repeat.key === "yearly") {
    newTask.date = new Date(newTask.date.getFullYear() + 1, newTask.date.getMonth(), newTask.date.getDate());
    }
    if (task.repeat.key === "monthly") {
    newTask.date = new Date(newTask.date.getFullYear(), newTask.date.getMonth() + 1, newTask.date.getDate());
    }
    if (task.repeat.key === "weekly") {
    newTask.date = new Date(newTask.date.getFullYear(), newTask.date.getMonth(), newTask.date.getDate() + 7);
    }
    if (task.repeat.key === "daily") {
    newTask.date = new Date(newTask.date.getFullYear(), newTask.date.getMonth(), newTask.date.getDate() + 1);
    }
    if (task.repeat.key === "everyn") {
    if (task.repeat.freq === "days") {
      newTask.date = new Date(newTask.date.getFullYear(), newTask.date.getMonth(), newTask.date.getDate() + task.repeat.n);
    }
    if (task.repeat.freq === "weeks") {
      newTask.date = new Date(newTask.date.getFullYear(), newTask.date.getMonth(), newTask.date.getDate() + task.repeat.n * 7);
    }
    if (task.repeat.freq === "months") {
      newTask.date = new Date(newTask.date.getFullYear(), newTask.date.getMonth() + task.repeat.n, newTask.date.getDate());
    }
    }
    if (task.repeat.key === "everynth") {
      const n = nthToN(newTask.repeat.nth);
      const dayOfWeek = dayOfWeekToInt(task.repeat.day);
      console.log("n", n);
      newTask.date = getNthWeekday(newTask.date.getFullYear(), newTask.date.getMonth() + 1, dayOfWeek, n);
    }
  task.repeat = null;
  console.log("created repeat", newTask.date);
  return newTask;
}

function getNthWeekday(year, month, dayOfWeek, n) {
  if (n < 0) {
    console.error("n is negative", n);
    throw new Error("n is negative");
  }
  console.log("nth", n);
  const firstOfMonth = new Date(year, month, 1);
  const firstDay = firstOfMonth.getDay(); // 0 (Sunday) to 6 (Saturday)
  console.log("firstDay", firstDay);
  console.log("dayOfWeek", dayOfWeek);
  const offset = (dayOfWeek - firstDay + 7) % 7;
  console.log("offset", offset);
  const day = 1 + offset + (n - 1) * 7;
  const result = new Date(year, month, day);
  console.log("result", year, month, day);
  return result.getMonth() === month ? result : getNthWeekday(year, month, dayOfWeek, n-1);
}

function nthToN(nth) {
  if (nth === "1st") {
    return 1;
  }
  if (nth === "2nd") {
    return 2;
  }
  if (nth === "3rd") {
    return 3;
  }
  if (nth === "4th") {
    return 4;
  }
  return 5;
}

function dayOfWeekToInt(day) {
  if (day === "monday") {
    return 1;
  }
  if (day === "tuesday") {
    return 2;
  }
  if (day === "wednesday") {
    return 3;
  }
  if (day === "thursday") {
    return 4;
  }
  if (day === "friday") {
    return 5;
  }
  if (day === "saturday") {
    return 6;
  }
  return 0;
}