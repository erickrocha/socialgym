export const updateObject = (oldObject, updatedProperties) => {
    return {
        ...oldObject,
        ...updatedProperties
    };
};

export const pageBuilder = data => {
    return {
        loading: false,
        data: data.content,
        page: {
            number: data.pageable.number,
            size: data.pageable.size,
            sort: data.pageable.sort,
            mode: data.pageable.mode,
            totalSize: data.totalSize,
        }
    };
};

export const paramBuilder = params => {
    let queryString = '?';
    Object.keys(params).forEach(key => {
        if (params[key] !== null) {
            queryString = queryString.concat(`${key}=${params[key]}&`);
        }
    });
    return queryString;
};

export const findOne = (data, id) => {
    return data.find(item => item.id === id);
};

export const findOneByField = (data, field, id) => {
    return data.find(item => item.key === id);
};

export const findIndex = (data, id) => {
    return data.findIndex(item => item.id === id);
};

export const check = (data, id) => {
    const item = findOne(data, id);
    const index = findIndex(data, id);
    item.selected = !item.selected;
    data[index] = item;
    return data;
};

export const checkAll = (data, selected = true) => {
    data.forEach(element => {
        element.selected = selected;
    });
    return data;
};

export const selected = (data) => {
    return data.filter(item => item.selected);
};

export const checkValidity = (value, rules) => {
    let isValid = true;
    if (!rules) {
        return true;
    }

    if (rules.required) {
        isValid = value.trim() !== '' && isValid;
    }

    if (rules.minLength) {
        isValid = value.length >= rules.minLength && isValid;
    }

    if (rules.maxLength) {
        isValid = value.length <= rules.maxLength && isValid;
    }

    if (rules.isEmail) {
        const pattern = /[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?/;
        isValid = pattern.test(value) && isValid;
    }

    if (rules.isNumeric) {
        const pattern = /^\d+$/;
        isValid = pattern.test(value) && isValid;
    }

    return isValid;
};

export const checkDay = value => {
    if (value < 10) {
        return `0${value}`;
    }
    return value;
};

export const buildChart = theme => {
    const options = {
        legend: {
            display: true
        },
        responsive: true,
        maintainAspectRatio: false,
        animation: false,
        cutoutPercentage: 50,
        layout: { padding: 0 },
        tooltips: {
            enabled: false,
            mode: 'index',
            intersect: true,
            borderWidth: 1,
            borderColor: theme.palette.divider,
            backgroundColor: theme.palette.white,
            titleFontColor: theme.palette.text.primary,
            bodyFontColor: theme.palette.text.secondary,
            footerFontColor: theme.palette.text.secondary
        }
    };
    return options;
};

export const hasOneChecked = (data, field) => {
    return data.find(item => item[field] === true) != null;
}

export const hasAllChecked = (data, field) => {
    let count = 0;
    data.forEach(item => {
        if (item[field] === true) {
            count++;
        }
    })
    return data.length === count;
}

