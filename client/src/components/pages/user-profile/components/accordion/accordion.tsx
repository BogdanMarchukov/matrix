import classes from './accordion.module.css'
import {AccordionItem} from "../../../../atoms/accordion-item/accordionItem";
import {BonusIcon} from "../../icons/bonusIcon";
import {TrainingIcon} from "../../icons/trainingIcon";
import {TestsIcon} from "../../icons/testsIcon";
import {CommunityIcon} from "../../icons/communityIcon";
import {TeamIcon} from "../../icons/teamIcon";
import {NewsIcon} from "../../icons/newsIcon";
import {AboutIcon} from "../../icons/aboutIcon";


const {
  root,
  accordion
} = classes;

const navigateList = [
  {
    icon: BonusIcon,
    text: "Бонусы",
    url: "Бонусы"
  },
  {
    icon: TrainingIcon,
    text: "Обучение",
    url: "Обучение"
  },
  {
    icon: TestsIcon,
    text: "Тесты",
    url: "Тесты"
  },
  {
    icon: CommunityIcon,
    text: "Сообщество",
    url: "Сообщество"
  },
  {
    icon: TeamIcon,
    text: "Команда",
    url: "Команда"
  },
  {
    icon: NewsIcon,
    text: "Новости",
    url: "Новости"
  },
  {
    icon: AboutIcon,
    text: "О нас",
    url: "О нас"
  }
]

export const Accordion = () => {
  return (
    <div className={root}>
      <ul className={accordion}>
        {navigateList.map((item) => <AccordionItem item={item} key={item.url}/>)}
      </ul>
    </div>
  )
}