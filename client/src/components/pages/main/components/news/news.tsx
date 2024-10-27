import {New} from "./components/new/new";
import classes from './news.module.css'
import newsFirst from './png/news-first.png';
import newsSecond from './png/news-second.png';
import newsThird from './png/news-third.png';

const newsList = [
  {
    id: '1',
    title: 'Новост 1',
    img: newsFirst,
  },
  {
    id: '2',
    title: 'Новост 2',
    img: newsSecond,
  },
  {
    id: '3',
    title: 'Новост 3',
    img: newsThird,
  }
]


export const News = () => {
  return (
    <div className={classes.rootBox}>
      <div className={classes.contentBox}>
        {newsList.map((newItem) => (
          <New newItem={newItem} key={newItem.id}/>
        ))}
      </div>
    </div>
  )
}