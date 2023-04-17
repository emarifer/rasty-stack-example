export const convertDate = (date: string): string => {
  const dt = new Date(Date.parse(date));

  const year = dt.getFullYear();
  const month = (dt.getMonth() + 1).toString().padStart(2, "0");
  const day = dt.getDate().toString().padStart(2, "0");
  const hour = dt.getHours().toString().padStart(2, "0");
  const min = dt.getMinutes().toString().padStart(2, "0");

  return `${hour}:${min} • ${day}-${month}-${year}`;
};

/*
 * https://javascript.info/date#:~:text=Date.parse%20from%20a%20string&text=The%20string%20format%20should%20be,%2C%20minutes%2C%20seconds%20and%20milliseconds.
 * https://stackoverflow.com/questions/6040515/how-do-i-get-month-and-date-of-javascript-in-2-digit-format
 */
