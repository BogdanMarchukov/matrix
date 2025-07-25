import classes from './new.module.css'

export interface New {
  id: string;
  title: string;
  img: string;
}

interface NewProps {
  newItem: New
}

const linearRadialGradientsList = ['first', 'second', 'third']

export const New = ({newItem}: NewProps) => {
  const {title, img} = newItem;

  const randomClassName = Math.floor(Math.random() * linearRadialGradientsList.length);

  return (
    <div className={`${classes.root} ${classes[linearRadialGradientsList[randomClassName]]}`}>
      <p className={classes.title}>{title}</p>
      <img className={classes.img} src={img} alt={title}/>
    </div>
  )
}