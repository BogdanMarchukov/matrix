import { FindByUserIdQuery } from '../../../../../../../__generated__/graphql';
import classes from './new.module.css'

interface NewProps {
  newItem: FindByUserIdQuery['userNews']['findByUserId'][number]
}

const linearRadialGradientsList = ['first', 'second', 'third']

export const New = ({ newItem }: NewProps) => {
  const { title, img } = newItem

  const randomClassName = Math.floor(Math.random() * linearRadialGradientsList.length);

  return (
    <div className={`${classes.root} ${classes[linearRadialGradientsList[randomClassName]]}`}>
      <p className={classes.title}>{title}</p>
      <img className={classes.img} src={img!} alt={title} />
    </div>
  )
}
