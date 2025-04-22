export function setToday(model) {
  if (model.date && model.preTtlDate === null) {
    model.preTtlDate = new Date(model.date);
  }
  model.date = getToday();
  return model;
}
export function setTomorrow(model) {
  if (model.date && model.date !== getToday() && model.preTtlDate === null) {
    model.preTtlDate = new Date(model.date);
  }
  model.date = getTomorrow();
  return model;
}
export function setLater(model) {
  if (model.preTtlDate) {
    model.date = new Date(model.preTtlDate);
    model.preTtlDate = null;
    return model;
  }
  model.date = null;
  return model;
}

export function getTtl(date) {
  const today = new Date();
  today.setHours(23, 59, 59, 999);
  const tomorrow = new Date(today);
  tomorrow.setDate(tomorrow.getDate() + 1);
  tomorrow.setHours(23, 59, 59, 999);
  if (!date) {
    return 'later';
  }
  if (typeof date === 'string' || typeof date === 'number') {
    date = new Date(date);
  }
  if (date <= today) {
    return 'today';
  } else if (date > tomorrow) {
    return 'later';
  } else {
    return 'tomorrow';
  }
}

export function getToday() {
  let date = new Date();
  date.setHours(0, 0, 0, 0);
  return date;
}

export function getTomorrow() {
  let date = new Date();
  date.setDate(date.getDate() + 1);
  date.setHours(0, 0, 0, 0);
  return date;
}