import {useState} from 'react';
import './CheckBoxGroup.scss';

const CheckBoxGroup = ({items, onChange, title, initialValues = [] }) => {
    const [selectedItems, setSelectedItems] = useState(initialValues);

    const handleSelectItem = (event, item) => {
        if (event.target.checked) {
            selectedItems.push(item);
            setSelectedItems([...selectedItems]);
        }else {
            setSelectedItems(selectedItems.filter(selectedItem => selectedItem !== item))
        }
        onChange(selectedItems);
    }

    return (
        <div className="CheckBoxGroup">
            <div className="CheckBoxGroup__muscle-groups">
                <label className="CheckBoxGroup__label">
                    {title}
                </label>
                <div className="CheckBoxGroup__checkbox-group">
                    {items.map((option) => (
                        <label key={option.value} className="CheckBoxGroup__checkbox-item">
                            <input
                                id={option.value}
                                name={option.value}
                                type="checkbox"
                                value={option.value}
                                checked={selectedItems.includes(option.value)}
                                onChange={e => handleSelectItem(e, option.value)}
                            />
                            <span>{option.label}</span>
                        </label>
                    ))}
                </div>
            </div>

        </div>
    )
}

export default CheckBoxGroup;