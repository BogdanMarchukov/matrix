import classes from './accordionItem.module.css';
import {ElementType} from "react";
import {NextIcon} from "../../pages/user-profile/icons/nextIcon";

export interface IAccordionItem {
  icon: ElementType;
  text: string;
  url: string;
}

interface AccordionItemProps {
  item: IAccordionItem;
}

export const AccordionItem = ({item}: AccordionItemProps) => {
  const Icon = item.icon;
  const onClick = () => console.log('url', item.url);

  return (
    <li className={classes.accordionItem} onClick={onClick}>
      <div className={classes.textBox}>
        <Icon animate={false}/>
        <p className={classes.text}>{item.text}</p>
      </div>
      <NextIcon/>
    </li>
  )
}